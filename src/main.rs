use minifb::{Key, Window, WindowOptions};
use rust_chip8_emulator::constants::{SCALE, WINDOW_HEIGHT, WINDOW_WIDTH};
use rust_chip8_emulator::cpu::CPU;
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

    let mut window = Window::new(
        "CHIP-8 Emulator - Rust",
        WINDOW_WIDTH * SCALE,
        WINDOW_HEIGHT * SCALE,
        WindowOptions::default(),
    )
    .expect("无法创建窗口");

    let mut buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    let cycles_per_frame: usize = 10;
    let mut last_instant = Instant::now();
    let mut timer_acc = Duration::from_secs(0);
    let timer_step = Duration::from_nanos(1_000_000_000u64 / 60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        let frame_dt = now.saturating_duration_since(last_instant);
        last_instant = now;
        timer_acc += frame_dt;

        update_keypad(&window, &mut cpu);

        for _ in 0..cycles_per_frame {
            cpu.tick();
        }

        while timer_acc >= timer_step {
            cpu.tick_timers();
            timer_acc -= timer_step;
        }

        let disp = cpu.display();
        let mut i = 0;
        for y in 0..WINDOW_HEIGHT {
            for x in 0..WINDOW_WIDTH {
                buffer[i] = if disp[y][x] {
                    0xFFFFFFFF
                } else {
                    0x00000000
                };
                i += 1;
            }
        }

        window
            .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }
}

fn update_keypad(window: &Window, cpu: &mut CPU) {
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
