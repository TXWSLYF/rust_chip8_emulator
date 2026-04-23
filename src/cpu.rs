use crate::constants::{
    FONT_DATA, FONT_SIZE, FONT_START_ADDRESS, MEMORY_SIZE, PROGRAM_START_ADDRESS,
};
use crate::display::Display;

pub struct CPU {
    // 内存
    memory: [u8; MEMORY_SIZE],

    // 16 个 8 位通用寄存器
    registers: [u8; 16],

    // 栈和栈指针
    stack: [u16; 16],
    stack_pointer: usize,

    // 程序计数器
    program_counter: usize,

    // 延迟计时器
    delay_timer: u8,

    // 声音计时器
    sound_timer: u8,

    // 显示器
    display: Display,
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu = Self {
            memory: [0; MEMORY_SIZE],
            registers: [0; 16],
            stack: [0; 16],
            stack_pointer: 0,
            program_counter: PROGRAM_START_ADDRESS,
            delay_timer: 0,
            sound_timer: 0,
            display: Display::new(),
        };
        cpu.memory[FONT_START_ADDRESS..FONT_START_ADDRESS + FONT_SIZE].copy_from_slice(&FONT_DATA);
        cpu
    }
}

impl CPU {
    pub fn load_rom(&mut self, rom: &[u8]) -> Result<String, String> {
        let start = PROGRAM_START_ADDRESS;
        let end = start + rom.len();

        if end > MEMORY_SIZE {
            return Err(format!(
                "ROM 文件过大：大小为 {} 字节，但可用空间仅剩 {} 字节",
                rom.len(),
                MEMORY_SIZE - start
            ));
        }

        self.memory[start..end].copy_from_slice(rom);

        Ok(format!("ROM 文件加载成功，大小为 {} 字节", rom.len()))
    }
}
