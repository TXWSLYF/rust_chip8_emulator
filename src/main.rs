mod constants;
mod cpu;

use constants::{SCALE, WINDOW_HEIGHT, WINDOW_WIDTH};
use cpu::CPU;
use minifb::{Key, Window, WindowOptions};
use std::fs;

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

    // 3. 模拟器主循环
    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.tick();

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
