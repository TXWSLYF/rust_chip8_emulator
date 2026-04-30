import { useEffect, useMemo, useRef, useState } from 'react';
import init, { WasmEmulator } from '../pkg/rust_chip8_emulator.js';

const WIDTH = 64;
const HEIGHT = 32;
const SCALE = 10;
const TIMER_STEP_MS = 1000 / 60;
const CYCLES_PER_FRAME = 10;

const KEY_MAP: Record<string, number> = {
  Digit1: 0x1,
  Digit2: 0x2,
  Digit3: 0x3,
  Digit4: 0xc,
  KeyQ: 0x4,
  KeyW: 0x5,
  KeyE: 0x6,
  KeyR: 0xd,
  KeyA: 0x7,
  KeyS: 0x8,
  KeyD: 0x9,
  KeyF: 0xe,
  KeyZ: 0xa,
  KeyX: 0x0,
  KeyC: 0xb,
  KeyV: 0xf,
};

const KEY_LAYOUT = [
  { physical: '1', chip: '1', code: 'Digit1' },
  { physical: '2', chip: '2', code: 'Digit2' },
  { physical: '3', chip: '3', code: 'Digit3' },
  { physical: '4', chip: 'C', code: 'Digit4' },
  { physical: 'Q', chip: '4', code: 'KeyQ' },
  { physical: 'W', chip: '5', code: 'KeyW' },
  { physical: 'E', chip: '6', code: 'KeyE' },
  { physical: 'R', chip: 'D', code: 'KeyR' },
  { physical: 'A', chip: '7', code: 'KeyA' },
  { physical: 'S', chip: '8', code: 'KeyS' },
  { physical: 'D', chip: '9', code: 'KeyD' },
  { physical: 'F', chip: 'E', code: 'KeyF' },
  { physical: 'Z', chip: 'A', code: 'KeyZ' },
  { physical: 'X', chip: '0', code: 'KeyX' },
  { physical: 'C', chip: 'B', code: 'KeyC' },
  { physical: 'V', chip: 'F', code: 'KeyV' },
] as const;

const BUILTIN_ROMS = [
  { id: 'chip8-logo', name: 'CHIP-8 Logo', url: '/1-chip8-logo.ch8' },
  { id: 'ibm-logo', name: 'IBM Logo', url: '/2-ibm-logo.ch8' },
  { id: 'corax', name: 'Corax+ (Test)', url: '/3-corax+.ch8' },
  { id: 'flags', name: 'Flags', url: '/4-flags.ch8' },
  { id: 'quirks', name: 'Quirks (Test)', url: '/5-quirks.ch8' },
  { id: 'breakout', name: 'BR8KOUT', url: '/br8kout.ch8' },
] as const;

type StatusTone = 'idle' | 'loading' | 'ready' | 'error';

type Locale = 'zh' | 'en' | 'ja';

type StatusCode =
  | 'preparing'
  | 'idle'
  | 'running'
  | 'loading_rom'
  | 'init_failed'
  | 'load_failed';

type Messages = Record<string, string>;

const MESSAGES: Record<Locale, Messages> = {
  zh: {
    'lang.label': '语言',
    'lang.zh': '中文',
    'lang.en': 'English',
    'lang.ja': '日本語',

    'status.preparing': '正在准备运行环境…',
    'status.idle': '选择一个 ROM 开始。',
    'status.running': '运行中',
    'status.loading_rom': '正在加载：{name}',
    'status.init_failed': '初始化失败：{message}',
    'status.load_failed': '加载失败：{message}',

    'hero.title': 'CHIP-8',
    'hero.subtitle': '选择内置 ROM 或上传 ROM，即可开始运行。',
    'hero.keys': '1–4 / QWER / ASDF / ZXCV',

    'display.kicker': '显示',
    'display.title': '模拟器屏幕',
    'display.builtin': '内置',
    'display.run': '运行',
    'display.upload': '上传 ROM',
    'display.loaded': '已加载',
    'display.mobile_hint': '移动端请使用页面底部的虚拟按键进行操作。',
    'display.footer': '载入后自动运行。键位：{keys}（对应十六键）。',

    'controls.kicker': '控制',
    'controls.title': '键位映射',
    'controls.subtitle': '当前按下的按键会高亮显示。',

    'vk.kicker': '控制',
    'vk.title': '虚拟按键',
    'vk.collapse': '收起',
    'vk.expand': '展开按键',
    'vk.keys': '16 键',
  },
  en: {
    'lang.label': 'Language',
    'lang.zh': '中文',
    'lang.en': 'English',
    'lang.ja': '日本語',

    'status.preparing': 'Preparing…',
    'status.idle': 'Choose a ROM to start.',
    'status.running': 'Running',
    'status.loading_rom': 'Loading: {name}',
    'status.init_failed': 'Init failed: {message}',
    'status.load_failed': 'Load failed: {message}',

    'hero.title': 'CHIP-8',
    'hero.subtitle': 'Pick a built-in ROM or upload your own to play.',
    'hero.keys': '1–4 / QWER / ASDF / ZXCV',

    'display.kicker': 'Display',
    'display.title': 'Screen',
    'display.builtin': 'Built-in',
    'display.run': 'Run',
    'display.upload': 'Upload ROM',
    'display.loaded': 'Loaded',
    'display.mobile_hint': 'On mobile, use the virtual keypad at the bottom.',
    'display.footer': 'Auto-runs after loading. Keys: {keys}.',

    'controls.kicker': 'Controls',
    'controls.title': 'Key Map',
    'controls.subtitle': 'Pressed keys are highlighted.',

    'vk.kicker': 'Controls',
    'vk.title': 'Virtual Keypad',
    'vk.collapse': 'Collapse',
    'vk.expand': 'Show keypad',
    'vk.keys': '16 keys',
  },
  ja: {
    'lang.label': '言語',
    'lang.zh': '中文',
    'lang.en': 'English',
    'lang.ja': '日本語',

    'status.preparing': '準備中…',
    'status.idle': 'ROM を選択して開始。',
    'status.running': '実行中',
    'status.loading_rom': '読み込み中：{name}',
    'status.init_failed': '初期化失敗：{message}',
    'status.load_failed': '読み込み失敗：{message}',

    'hero.title': 'CHIP-8',
    'hero.subtitle': '内蔵 ROM を選ぶか、ROM をアップロードして開始。',
    'hero.keys': '1–4 / QWER / ASDF / ZXCV',

    'display.kicker': '表示',
    'display.title': '画面',
    'display.builtin': '内蔵',
    'display.run': '実行',
    'display.upload': 'ROM をアップロード',
    'display.loaded': '読み込み済み',
    'display.mobile_hint': 'モバイルでは下部の仮想キーパッドを使用してください。',
    'display.footer': '読み込み後に自動実行。キー：{keys}。',

    'controls.kicker': '操作',
    'controls.title': 'キー配置',
    'controls.subtitle': '押下中のキーがハイライトされます。',

    'vk.kicker': '操作',
    'vk.title': '仮想キーパッド',
    'vk.collapse': '折りたたむ',
    'vk.expand': 'キーパッド',
    'vk.keys': '16 キー',
  },
};

function detectLocale(): Locale {
  const stored = localStorage.getItem('chip8.locale');
  if (stored === 'zh' || stored === 'en' || stored === 'ja') return stored;
  const lang = navigator.language.toLowerCase();
  if (lang.startsWith('ja')) return 'ja';
  if (lang.startsWith('zh')) return 'zh';
  return 'en';
}

function formatMessage(template: string, vars?: Record<string, string>) {
  if (!vars) return template;
  return template.replaceAll(/\{(\w+)\}/g, (_, key) => vars[key] ?? `{${key}}`);
}

function useI18n() {
  const [locale, setLocale] = useState<Locale>(() => detectLocale());
  const messages = MESSAGES[locale];

  const t = (key: string, vars?: Record<string, string>) =>
    formatMessage(messages[key] ?? key, vars);

  const setAndPersistLocale = (next: Locale) => {
    setLocale(next);
    localStorage.setItem('chip8.locale', next);
    document.documentElement.lang = next === 'zh' ? 'zh-CN' : next;
  };

  return { locale, setLocale: setAndPersistLocale, t };
}

const CHIP_KEYPAD = [
  { label: '1', value: 0x1 },
  { label: '2', value: 0x2 },
  { label: '3', value: 0x3 },
  { label: 'C', value: 0xc },
  { label: '4', value: 0x4 },
  { label: '5', value: 0x5 },
  { label: '6', value: 0x6 },
  { label: 'D', value: 0xd },
  { label: '7', value: 0x7 },
  { label: '8', value: 0x8 },
  { label: '9', value: 0x9 },
  { label: 'E', value: 0xe },
  { label: 'A', value: 0xa },
  { label: '0', value: 0x0 },
  { label: 'B', value: 0xb },
  { label: 'F', value: 0xf },
] as const;

function StatusPill({
  tone,
  children,
}: {
  tone: StatusTone;
  children: string;
}) {
  const toneClass: Record<StatusTone, string> = {
    idle: 'border-white/10 bg-white/5 text-slate-300',
    loading: 'border-sky-400/30 bg-sky-400/10 text-sky-200',
    ready: 'border-emerald-400/30 bg-emerald-400/10 text-emerald-200',
    error: 'border-rose-400/30 bg-rose-400/10 text-rose-200',
  };

  return (
    <span
      className={`inline-flex items-center rounded-full border px-3 py-1 text-xs font-medium tracking-[0.24em] uppercase ${toneClass[tone]}`}
    >
      {children}
    </span>
  );
}

function VirtualKeypad({
  visible,
  pressedKeys,
  onSetKey,
  onToggle,
  t,
}: {
  visible: boolean;
  pressedKeys: ReadonlySet<number>;
  onSetKey: (key: number, pressed: boolean) => void;
  onToggle: () => void;
  t: (key: string, vars?: Record<string, string>) => string;
}) {
  return (
    <div className="sm:hidden">
      <div className="fixed inset-x-0 bottom-0 z-50">
        {!visible ? (
          <div className="pointer-events-auto mx-3 mb-3 flex justify-center">
            <button
              type="button"
              onClick={onToggle}
              className="panel inline-flex items-center gap-2 rounded-full px-5 py-3 text-sm font-medium text-white"
            >
              {t('vk.expand')}
              <span className="text-xs uppercase tracking-[0.22em] text-slate-400">
                {t('vk.keys')}
              </span>
            </button>
          </div>
        ) : (
          <>
            <div className="pointer-events-none h-10 bg-linear-to-t from-black/40 to-transparent" />

            <div className="panel pointer-events-auto mx-3 mb-3 rounded-[28px] p-3">
              <div className="mb-3 flex items-center justify-between gap-3">
                <div className="min-w-0">
                  <div className="text-xs uppercase tracking-[0.22em] text-slate-400">
                    {t('vk.kicker')}
                  </div>
                  <div className="truncate text-sm font-medium text-white">{t('vk.title')}</div>
                </div>
                <button
                  type="button"
                  onClick={onToggle}
                  className="rounded-full border border-white/10 bg-white/5 px-4 py-2 text-xs font-medium text-slate-200"
                >
                  {t('vk.collapse')}
                </button>
              </div>

              <div className="grid grid-cols-4 gap-2">
                {CHIP_KEYPAD.map((key) => {
                  const active = pressedKeys.has(key.value);
                  return (
                    <button
                      key={key.value}
                      type="button"
                      className={`select-none rounded-2xl border py-3 text-center text-lg font-semibold tracking-wide transition active:scale-[0.98] ${
                        active
                          ? 'border-sky-300/70 bg-sky-400/20 text-sky-100 shadow-[0_0_30px_rgba(56,189,248,0.18)]'
                          : 'border-white/10 bg-white/5 text-slate-200'
                      }`}
                      onPointerDown={(event) => {
                        event.preventDefault();
                        (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId);
                        onSetKey(key.value, true);
                      }}
                      onPointerUp={(event) => {
                        event.preventDefault();
                        onSetKey(key.value, false);
                      }}
                      onPointerCancel={() => onSetKey(key.value, false)}
                      onPointerLeave={() => onSetKey(key.value, false)}
                    >
                      {key.label}
                    </button>
                  );
                })}
              </div>
            </div>
          </>
        )}
      </div>
    </div>
  );
}

export default function App() {
  const { locale, setLocale, t } = useI18n();
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const emuRef = useRef<WasmEmulator | null>(null);
  const memoryRef = useRef<WebAssembly.Memory | null>(null);
  const offscreenCanvasRef = useRef<HTMLCanvasElement | null>(null);
  const offscreenContextRef = useRef<CanvasRenderingContext2D | null>(null);
  const imageDataRef = useRef<ImageData | null>(null);
  const animationFrameRef = useRef<number>(0);
  const lastTimestampRef = useRef<number | null>(null);
  const timerAccumulatorRef = useRef<number>(0);

  const [wasmReady, setWasmReady] = useState(false);
  const [romName, setRomName] = useState('未加载');
  const [pressedCodes, setPressedCodes] = useState<Set<string>>(() => new Set());
  const [pressedChipKeys, setPressedChipKeys] = useState<Set<number>>(() => new Set());
  const [selectedBuiltin, setSelectedBuiltin] = useState<string>(
    BUILTIN_ROMS[0]?.id ?? '',
  );
  const [virtualKeypadOpen, setVirtualKeypadOpen] = useState(true);
  const [status, setStatus] = useState<{
    tone: StatusTone;
    code: StatusCode;
    vars?: Record<string, string>;
  }>({
    tone: 'loading',
    code: 'preparing',
  });

  const keyHint = useMemo(() => t('hero.keys'), [t]);
  const statusLabel = useMemo(() => {
    const vars = status.vars;
    if (status.code === 'loading_rom' && vars?.name) {
      return t('status.loading_rom', { name: vars.name });
    }
    if (status.code === 'init_failed' && vars?.message) {
      return t('status.init_failed', { message: vars.message });
    }
    if (status.code === 'load_failed' && vars?.message) {
      return t('status.load_failed', { message: vars.message });
    }
    return t(`status.${status.code}`);
  }, [status.code, status.vars, t]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    canvas.width = WIDTH * SCALE;
    canvas.height = HEIGHT * SCALE;

    const offscreenCanvas = document.createElement('canvas');
    offscreenCanvas.width = WIDTH;
    offscreenCanvas.height = HEIGHT;

    const offscreenContext = offscreenCanvas.getContext('2d');
    if (!offscreenContext) {
      setStatus({ tone: 'error', code: 'init_failed', vars: { message: '2D Canvas unavailable' } });
      return;
    }

    offscreenCanvasRef.current = offscreenCanvas;
    offscreenContextRef.current = offscreenContext;
    imageDataRef.current = offscreenContext.createImageData(WIDTH, HEIGHT);
  }, []);

  useEffect(() => {
    let disposed = false;

    async function bootWasm() {
      try {
        const wasm = await init();
        if (disposed) return;
        memoryRef.current = wasm.memory as WebAssembly.Memory;
        setWasmReady(true);
        setStatus({ tone: 'idle', code: 'idle' });
      } catch (error) {
        if (disposed) return;
        setStatus({
          tone: 'error',
          code: 'init_failed',
          vars: { message: error instanceof Error ? error.message : String(error) },
        });
      }
    }

    void bootWasm();

    return () => {
      disposed = true;
      if (animationFrameRef.current) cancelAnimationFrame(animationFrameRef.current);
      if (emuRef.current) {
        emuRef.current.free();
        emuRef.current = null;
      }
    };
  }, []);

  function startEmulator(bytes: Uint8Array, name: string) {
    const emulator = new WasmEmulator();
    emulator.load_rom(bytes);

    if (emuRef.current) {
      emuRef.current.free();
    }

    emuRef.current = emulator;
    lastTimestampRef.current = null;
    timerAccumulatorRef.current = 0;
    setPressedCodes(new Set());
    setPressedChipKeys(new Set());
    setRomName(name);
    setStatus({ tone: 'ready', code: 'running' });
  }

  function setChipKey(key: number, pressed: boolean) {
    if (emuRef.current) {
      emuRef.current.set_key(key, pressed);
    }
    setPressedChipKeys((previous) => {
      const next = new Set(previous);
      if (pressed) {
        next.add(key);
      } else {
        next.delete(key);
      }
      return next;
    });
  }

  async function loadBuiltinRom() {
    if (!wasmReady) return;

    const chosen = BUILTIN_ROMS.find((rom) => rom.id === selectedBuiltin);
    if (!chosen) return;

    try {
      setStatus({ tone: 'loading', code: 'loading_rom', vars: { name: chosen.name } });
      const response = await fetch(chosen.url);
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }
      const bytes = new Uint8Array(await response.arrayBuffer());
      startEmulator(bytes, chosen.name);
    } catch (error) {
      setStatus({
        tone: 'error',
        code: 'load_failed',
        vars: { message: error instanceof Error ? error.message : String(error) },
      });
    }
  }

  useEffect(() => {
    function drawFrame() {
      const emu = emuRef.current;
      const canvas = canvasRef.current;
      const offscreenContext = offscreenContextRef.current;
      const imageData = imageDataRef.current;
      const memory = memoryRef.current;

      if (!emu || !canvas || !offscreenContext || !imageData || !memory) return;

      emu.sync_rgba();

      const ptr = emu.rgbaPtr();
      const len = emu.rgbaLen();
      const rgbaBuffer = new Uint8Array(memory.buffer, ptr, len);
      imageData.data.set(rgbaBuffer);
      offscreenContext.putImageData(imageData, 0, 0);

      const context = canvas.getContext('2d');
      if (!context) return;
      context.imageSmoothingEnabled = false;
      context.clearRect(0, 0, canvas.width, canvas.height);
      if (offscreenCanvasRef.current) {
        context.drawImage(offscreenCanvasRef.current, 0, 0, WIDTH * SCALE, HEIGHT * SCALE);
      }
    }

    function frame(timestamp: number) {
      animationFrameRef.current = requestAnimationFrame(frame);

      const emu = emuRef.current;
      if (!emu) return;

      if (lastTimestampRef.current === null) {
        lastTimestampRef.current = timestamp;
      }

      const delta = timestamp - lastTimestampRef.current;
      lastTimestampRef.current = timestamp;
      timerAccumulatorRef.current += delta;

      for (let i = 0; i < CYCLES_PER_FRAME; i += 1) {
        emu.tick();
      }

      while (timerAccumulatorRef.current >= TIMER_STEP_MS) {
        emu.tick_timers();
        timerAccumulatorRef.current -= TIMER_STEP_MS;
      }

      drawFrame();
    }

    animationFrameRef.current = requestAnimationFrame(frame);
    return () => cancelAnimationFrame(animationFrameRef.current);
  }, []);

  useEffect(() => {
    function updateKey(code: string, pressed: boolean) {
      const mapped = KEY_MAP[code];
      if (mapped === undefined) return false;

      setChipKey(mapped, pressed);

      setPressedCodes((previous) => {
        const next = new Set(previous);
        if (pressed) {
          next.add(code);
        } else {
          next.delete(code);
        }
        return next;
      });

      return true;
    }

    function onKeyDown(event: KeyboardEvent) {
      if (updateKey(event.code, true)) {
        event.preventDefault();
      }
    }

    function onKeyUp(event: KeyboardEvent) {
      if (updateKey(event.code, false)) {
        event.preventDefault();
      }
    }

    window.addEventListener('keydown', onKeyDown);
    window.addEventListener('keyup', onKeyUp);

    return () => {
      window.removeEventListener('keydown', onKeyDown);
      window.removeEventListener('keyup', onKeyUp);
    };
  }, []);

  async function handleRomChange(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0];
    if (!file) return;

    try {
      const bytes = new Uint8Array(await file.arrayBuffer());
      setStatus({ tone: 'loading', code: 'loading_rom', vars: { name: file.name } });
      startEmulator(bytes, file.name);
    } catch (error) {
      setStatus({
        tone: 'error',
        code: 'load_failed',
        vars: { message: error instanceof Error ? error.message : String(error) },
      });
    } finally {
      event.target.value = '';
    }
  }

  return (
    <main className="min-h-screen px-3 py-5 pb-60 text-slate-100 sm:px-6 sm:pb-6 lg:px-8">
      <div className="mx-auto flex w-full max-w-7xl flex-col gap-6">
        <section className="panel overflow-hidden rounded-[32px] p-6 sm:p-8">
          <div className="flex flex-col gap-8 lg:flex-row lg:items-end lg:justify-between">
            <div className="max-w-3xl">
              <div className="mb-4 flex flex-wrap items-center gap-3">
                <StatusPill tone={status.tone}>{statusLabel}</StatusPill>
                <span className="rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-slate-300">
                  {keyHint}
                </span>
                <div className="flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-1 text-xs text-slate-300">
                  <span className="text-slate-400">{t('lang.label')}</span>
                  <select
                    value={locale}
                    onChange={(e) => setLocale(e.target.value as Locale)}
                    className="bg-transparent outline-none"
                  >
                    <option value="zh" className="bg-slate-950 text-white">
                      {t('lang.zh')}
                    </option>
                    <option value="en" className="bg-slate-950 text-white">
                      {t('lang.en')}
                    </option>
                    <option value="ja" className="bg-slate-950 text-white">
                      {t('lang.ja')}
                    </option>
                  </select>
                </div>
              </div>

              <h1 className="text-4xl font-semibold tracking-tight text-white sm:text-5xl">
                {t('hero.title')}
              </h1>
              <p className="mt-4 max-w-2xl text-sm leading-7 text-slate-300 sm:text-base">
                {t('hero.subtitle')}
              </p>
            </div>
          </div>
        </section>

        <section className="grid gap-6 xl:grid-cols-[minmax(0,1.6fr)_minmax(320px,0.9fr)]">
          <div className="panel rounded-[32px] p-4 sm:p-6">
            <div className="mb-5 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
              <div>
                <p className="text-xs uppercase tracking-[0.22em] text-sky-200">
                  {t('display.kicker')}
                </p>
                <h2 className="mt-2 text-2xl font-semibold text-white">{t('display.title')}</h2>
              </div>

              <div className="flex flex-col gap-3 sm:flex-row sm:items-center">
                <div className="flex items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-2">
                  <span className="text-xs uppercase tracking-[0.22em] text-slate-400">
                    {t('display.builtin')}
                  </span>
                  <select
                    className="w-44 bg-transparent text-sm text-white outline-none"
                    value={selectedBuiltin}
                    onChange={(e) => setSelectedBuiltin(e.target.value)}
                    disabled={!wasmReady}
                  >
                    {BUILTIN_ROMS.map((rom) => (
                      <option key={rom.id} value={rom.id} className="bg-slate-950 text-white">
                        {rom.name}
                      </option>
                    ))}
                  </select>
                </div>

                <button
                  type="button"
                  onClick={loadBuiltinRom}
                  disabled={!wasmReady}
                  className="inline-flex items-center justify-center rounded-full bg-white/10 px-5 py-3 text-sm font-medium text-white transition hover:bg-white/15 disabled:cursor-not-allowed disabled:opacity-60"
                >
                  {t('display.run')}
                </button>

                <label className="inline-flex cursor-pointer items-center justify-center rounded-full bg-sky-400 px-5 py-3 text-sm font-medium text-slate-950 transition hover:bg-sky-300 disabled:cursor-not-allowed disabled:opacity-60">
                  <input
                    type="file"
                    accept="*/*"
                    className="hidden"
                    disabled={!wasmReady}
                    onChange={handleRomChange}
                  />
                  {t('display.upload')}
                </label>
              </div>
            </div>

            <div className="-mx-4 rounded-[28px] border border-white/10 bg-slate-950/80 p-2 sm:mx-0 sm:p-4">
              <canvas
                ref={canvasRef}
                className="pixel-screen aspect-2/1 w-full rounded-2xl border border-sky-400/20 bg-black shadow-[0_0_80px_rgba(56,189,248,0.08)] sm:rounded-[20px]"
              />
            </div>

            <div className="mt-5 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
              <div>
                <p className="text-xs uppercase tracking-[0.22em] text-slate-500">
                  {t('display.loaded')}
                </p>
                <p className="mt-1 text-base font-medium text-white">{romName}</p>
              </div>
              <p className="max-w-md text-sm leading-6 text-slate-400">
                {t('display.footer', { keys: keyHint })}
              </p>
            </div>
            <p className="mt-4 text-sm text-slate-400 sm:hidden">
              {t('display.mobile_hint')}
            </p>
          </div>

          <div className="hidden flex-col gap-6 xl:flex">
            <section className="panel rounded-[32px] p-5 sm:p-6">
              <p className="text-xs uppercase tracking-[0.22em] text-violet-200">
                {t('controls.kicker')}
              </p>
              <h2 className="mt-2 text-2xl font-semibold text-white">{t('controls.title')}</h2>
              <p className="mt-3 text-sm leading-6 text-slate-400">{t('controls.subtitle')}</p>

              <div className="mt-5 grid grid-cols-4 gap-3">
                {KEY_LAYOUT.map((key) => {
                  const active = pressedCodes.has(key.code);
                  return (
                    <div
                      key={key.code}
                      className={`rounded-2xl border px-3 py-4 text-center transition ${
                        active
                          ? 'border-sky-300/70 bg-sky-400/20 text-sky-100 shadow-[0_0_30px_rgba(56,189,248,0.18)]'
                          : 'border-white/10 bg-white/5 text-slate-300'
                      }`}
                    >
                      <div className="text-lg font-semibold">{key.physical}</div>
                      <div className="mt-1 text-xs uppercase tracking-[0.2em] text-slate-400">
                        CHIP-{key.chip}
                      </div>
                    </div>
                  );
                })}
              </div>
            </section>
          </div>
        </section>
      </div>

      <VirtualKeypad
        visible={virtualKeypadOpen}
        pressedKeys={pressedChipKeys}
        onSetKey={setChipKey}
        onToggle={() => setVirtualKeypadOpen((v) => !v)}
        t={t}
      />
    </main>
  );
}
