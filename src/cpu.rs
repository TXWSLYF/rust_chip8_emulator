use crate::constants::{
    FONT_DATA, FONT_SIZE, FONT_START_ADDRESS, MEMORY_SIZE, PROGRAM_START_ADDRESS, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

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
    // 索引寄存器
    index_register: u16,

    // 延迟计时器
    delay_timer: u8,

    // 声音计时器
    sound_timer: u8,

    // 显示器
    pub display: [[bool; WINDOW_WIDTH]; WINDOW_HEIGHT],
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu = Self {
            memory: [0; MEMORY_SIZE],
            registers: [0; 16],
            stack: [0; 16],
            stack_pointer: 0,
            program_counter: PROGRAM_START_ADDRESS,
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [[false; WINDOW_WIDTH]; WINDOW_HEIGHT],
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

impl CPU {
    pub fn tick(&mut self) {
        // 1. Fetch
        let op_byte1 = self.memory[self.program_counter] as u16;
        let op_byte2 = self.memory[self.program_counter + 1] as u16;
        let opcode = (op_byte1 << 8) | op_byte2;

        // 2. Advance PC (指向下一条指令)
        // 注意：指令是 2 字节长，所以 PC 每次加 2
        self.program_counter += 2;

        // 3. Decode & Execute
        self.execute(opcode);
    }

    fn execute(&mut self, opcode: u16) {
        // 拆解 Opcode 的各个部分，方便匹配
        // 例如：0x1234 -> op=1, x=2, y=3, n=4, nn=34, nnn=234
        let op = (opcode & 0xF000) >> 12;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        let n = opcode & 0x000F;
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;

        match (op, x, y, n) {
            (0, 0, 0xE, 0) => {
                // 00E0: 清屏
                self.display = [[false; WINDOW_WIDTH]; WINDOW_HEIGHT];
            }
            (1, _, _, _) => {
                // 1NNN: 跳转到 NNN 地址
                self.program_counter = nnn as usize;
            }
            (0x6, _, _, _) => {
                // 6XNN: 加载 NN 到寄存器 X
                self.registers[x as usize] = nn;
            }
            (0xA, _, _, _) => {
                // ANNN: 加载 NNN 到 I 寄存器
                self.index_register = nnn;
            }
            (0xD, _, _, _) => {
                // DXYN: 绘图
                // % WINDOW_WIDTH 和 % WINDOW_HEIGHT 是为了确保坐标在显示范围内, 可以实现环绕显示
                let x_coord = self.registers[x as usize] as usize % WINDOW_WIDTH;
                let y_coord = self.registers[y as usize] as usize % WINDOW_HEIGHT;
                let height = n as usize;
            
                // 重置碰撞标志 VF 为 0
                self.registers[0xF] = 0;
            
                for row in 0..height {
                    // 从内存 I + row 处取出一个字节的数据（一行 8 像素）
                    let sprite_byte = self.memory[self.index_register as usize + row];
            
                    for col in 0..8 {
                        // 通过位运算提取这一字节里的每一位 (Bit)
                        // 0x80 是 10000000，用来检查最高位
                        if (sprite_byte & (0x80 >> col)) != 0 {
                            let dx = (x_coord + col) % WINDOW_WIDTH;
                            let dy = (y_coord + row) % WINDOW_HEIGHT;
            
                            // 检查是否发生碰撞（目标位置已经是 true）
                            if self.display[dy][dx] {
                                self.registers[0xF] = 1;
                            }
            
                            // 异或操作：true ^ true = false, false ^ true = true
                            self.display[dy][dx] ^= true;
                        }
                    }
                }
            }
            // ... 后续慢慢填充其他指令
            _ => todo!("未实现的指令: {:#06X}", opcode),
        }
    }
}
