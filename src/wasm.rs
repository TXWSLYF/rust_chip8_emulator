use wasm_bindgen::prelude::*;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::cpu::CPU;

/// 浏览器侧通过 wasm-bindgen 调用的薄封装。
#[wasm_bindgen]
pub struct WasmEmulator {
    cpu: CPU,
    rgba: Vec<u8>,
}

#[wasm_bindgen]
impl WasmEmulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmEmulator {
        WasmEmulator {
            cpu: CPU::new(),
            rgba: vec![0u8; WINDOW_WIDTH * WINDOW_HEIGHT * 4],
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), JsValue> {
        self.cpu.load_rom(rom).map_err(|e| JsValue::from_str(&e))?;
        Ok(())
    }

    pub fn tick(&mut self) {
        self.cpu.tick();
    }

    pub fn tick_many(&mut self, count: u32) {
        for _ in 0..count {
            self.cpu.tick();
        }
    }

    pub fn tick_timers(&mut self) {
        self.cpu.tick_timers();
    }

    /// `key` 为 0..=15，对应 CHIP-8 十六键。
    pub fn set_key(&mut self, key: u8, pressed: bool) {
        let k = key as usize;
        if k < 16 {
            self.cpu.set_key(k, pressed);
        }
    }

    /// 将当前显示写入 RGBA 缓冲。返回与 `rgba_ptr` 对应的字节长度。
    pub fn sync_rgba(&mut self) -> usize {
        let grid = self.cpu.display();
        let mut i = 0usize;
        for y in 0..WINDOW_HEIGHT {
            for x in 0..WINDOW_WIDTH {
                let on = grid[y][x];
                let v = if on { 255u8 } else { 0u8 };
                self.rgba[i] = v;
                self.rgba[i + 1] = v;
                self.rgba[i + 2] = v;
                self.rgba[i + 3] = 255;
                i += 4;
            }
        }
        self.rgba.len()
    }

    /// 指向最近一次 `sync_rgba` 写入的 RGBA 数据（位于 wasm 线性内存中）。
    #[wasm_bindgen(js_name = rgbaPtr)]
    pub fn rgba_ptr(&self) -> *const u8 {
        self.rgba.as_ptr()
    }

    #[wasm_bindgen(js_name = rgbaLen)]
    pub fn rgba_len(&self) -> usize {
        self.rgba.len()
    }
}
