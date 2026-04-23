mod constants;
mod cpu;

use constants::{SCALE, WINDOW_HEIGHT, WINDOW_WIDTH};
use cpu::CPU;
use minifb::{Key, Window, WindowOptions};
use std::fs;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rom_file>", args[0]);
        std::process::exit(1);
    }
    let rom_file = args[1].clone();
    let rom = fs::read(rom_file).expect("Failed to read ROM file");
    let mut cpu = CPU::new();

    match cpu.load_rom(&rom) {
        Ok(message) => println!("{}", message),
        Err(message) => eprintln!("{}", message),
    }

    // 1. 创建窗口
    let mut window = Window::new(
        "CHIP-8 Emulator - Rust",
        WINDOW_WIDTH * SCALE,
        WINDOW_HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .expect("无法创建窗口");

    // 2. 准备缓冲区（minifb 需要 u32 格式的 RGB 数据）
    // 我们的 CPU.display 是 bool，这里需要转换一下
    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    // 常见做法：每帧执行若干条指令；定时器按真实时间以 60Hz 递减
    let cycles_per_frame: usize = 10;
    let mut last_instant = Instant::now();
    let mut timer_acc = Duration::from_secs(0);
    let timer_step = Duration::from_nanos(1_000_000_000u64 / 60);

    // 3. 模拟器主循环
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // 统计本帧耗时，用于 60Hz 定时器
        let now = Instant::now();
        let frame_dt = now.saturating_duration_since(last_instant);
        last_instant = now;
        timer_acc += frame_dt;

        // 更新 CHIP-8 16 键键盘状态
        update_keypad(&window, &mut cpu);

        // 执行若干条 CPU 指令
        for _ in 0..cycles_per_frame {
            cpu.tick();
        }

        // 以 60Hz 更新 DT/ST
        while timer_acc >= timer_step {
            cpu.tick_timers();
            timer_acc -= timer_step;
        }

        // 4. 将 CPU 的 [[bool; 64]; 32] 转换为窗口的 u32 像素
        let mut i = 0;
        // 这里为了简单，先直接按像素对应（不考虑缩放的话，画面会很小）
        // 建议写一个转换函数把 CPU.display 的内容映射到 buffer

        for y in 0..WINDOW_HEIGHT {
            for x in 0..WINDOW_WIDTH {
                buffer[i] = if cpu.display[y][x] {
                    0xFFFFFFFF
                } else {
                    0x00000000
                };
                i += 1;
            }
        }

        // 5. 更新窗口画面
        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}

fn update_keypad(window: &Window, cpu: &mut CPU) {
    // 常见 CHIP-8 键位映射:
    // 1 2 3 4  -> 1 2 3 C
    // Q W E R  -> 4 5 6 D
    // A S D F  -> 7 8 9 E
    // Z X C V  -> A 0 B F
    let mapping: &[(Key, usize)] = &[
        (Key::Key1, 0x1),
        (Key::Key2, 0x2),
        (Key::Key3, 0x3),
        (Key::Key4, 0xC),
        (Key::Q, 0x4),
        (Key::W, 0x5),
        (Key::E, 0x6),
        (Key::R, 0xD),
        (Key::A, 0x7),
        (Key::S, 0x8),
        (Key::D, 0x9),
        (Key::F, 0xE),
        (Key::Z, 0xA),
        (Key::X, 0x0),
        (Key::C, 0xB),
        (Key::V, 0xF),
    ];

    for (host_key, chip8_key) in mapping {
        cpu.set_key(*chip8_key, window.is_key_down(*host_key));
    }
}
