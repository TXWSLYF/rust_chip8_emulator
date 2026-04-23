mod constants;
mod cpu;
mod display;

use cpu::CPU;
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
}
