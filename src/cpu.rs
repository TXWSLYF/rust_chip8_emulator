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
            (0x0, 0x0, 0xE, 0xE) => {
                // 00EE: Return from a subroutine
                if self.stack_pointer == 0 {
                    panic!("Stack Underflow! 栈下溢了，没有可以返回的地址。");
                }

                // 1. 栈指针下移（回到上一个存储位置）
                self.stack_pointer -= 1;
                // 2. 从栈里取出之前 2NNN 存进去的返回地址
                self.program_counter = self.stack[self.stack_pointer] as usize;
            }
            (1, _, _, _) => {
                // 1NNN: 跳转到 NNN 地址
                self.program_counter = nnn as usize;
            }
            (0x2, _, _, _) => {
                // 2NNN: Call subroutine at NNN
                if self.stack_pointer >= self.stack.len() {
                    panic!("Stack Overflow! 栈溢出了，函数调用太深了。");
                }

                // 1. 把当前 PC 存入栈顶。注意：现在的 PC 已经指向下一条指令了（如果你在 fetch 时加了 2）
                self.stack[self.stack_pointer] = self.program_counter as u16;
                // 2. 栈指针向上移
                self.stack_pointer += 1;
                // 3. PC 跳转到目标地址
                self.program_counter = nnn as usize;
            }
            (0x3, _, _, _) => {
                // 3XNN: Skip next instruction if Vx == NN
                let vx_value = self.registers[x as usize];
                if vx_value == nn {
                    self.program_counter += 2;
                }
            }
            (0x4, _, _, _) => {
                // 4XNN: Skip next instruction if Vx != NN
                if self.registers[x as usize] != nn {
                    self.program_counter += 2;
                }
            }
            (0x5, _, _, 0x0) => {
                // 5XY0: Skip next instruction if Vx == Vy
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.program_counter += 2;
                }
            }
            (0x6, _, _, _) => {
                // 6XNN: 加载 NN 到寄存器 X
                self.registers[x as usize] = nn;
            }
            (0x8, _, _, 0x0) => {
                // 8XY0: Set Vx = Vy
                self.registers[x as usize] = self.registers[y as usize];
            }
            (0x8, _, _, 0x1) => {
                // 8XY1: Set Vx = Vx | Vy
                self.registers[x as usize] |= self.registers[y as usize];
            }
            (0x8, _, _, 0x2) => {
                // 8XY2: Set Vx = Vx & Vy
                self.registers[x as usize] &= self.registers[y as usize];
                // println!("  -> [逻辑与] V{:X} &= V{:X}", x, y);
            }
            (0x9, _, _, 0x0) => {
                // 9XY0: Skip next instruction if Vx != Vy
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.program_counter += 2;
                }
            }
            (0x8, _, _, 0x3) => {
                // 8XY3: Set Vx = Vx ^ Vy
                self.registers[x as usize] ^= self.registers[y as usize];
            }
            (0x8, _, _, 0x4) => {
                // 8XY4: Vx = Vx + Vy, set VF = carry
                let vx = self.registers[x as usize];
                let vy = self.registers[y as usize];

                // 使用 Rust 提供的 overflowing_add
                let (sum, did_overflow) = vx.overflowing_add(vy);

                // 存回结果
                self.registers[x as usize] = sum;

                // 更新 VF 标志位
                // 注意：必须在更新 Vx 之后设置 VF，
                // 因为如果 x 是 0xF (虽然极少见)，Vx 的更新会覆盖 VF 的设置。
                self.registers[0xF] = if did_overflow { 1 } else { 0 };
            }
            (0x8, _, _, 0x5) => {
                // 8XY5: Vx = Vx - Vy, set VF = NOT borrow
                let vx = self.registers[x as usize];
                let vy = self.registers[y as usize];

                // overflowing_sub 返回 (结果, 是否借位)
                let (diff, did_borrow) = vx.overflowing_sub(vy);

                self.registers[x as usize] = diff;

                // 重要：如果没借位(did_borrow是false)，VF设为1
                self.registers[0xF] = if !did_borrow { 1 } else { 0 };
            }
            (0x8, _, _, 0x6) => {
                // 8XY6: Vx >>= 1
                let vx = self.registers[x as usize];

                // 1. 提取最低位存入 VF
                self.registers[0xF] = vx & 0x01;

                // 2. 右移一位并存回
                self.registers[x as usize] = vx >> 1;
            }
            (0x8, _, _, 0xE) => {
                // 8XYE: Vx <<= 1
                let vx = self.registers[x as usize];

                // 1. 提取最高位 (MSB) 存入 VF
                // 0x80 是二进制 10000000
                self.registers[0xF] = (vx >> 7) & 0x01;

                // 2. 左移一位并存回
                self.registers[x as usize] = vx << 1;
            }
            (0xF, _, 0x6, 0x5) => {
                // FX65: Fill registers V0 to Vx with values from memory starting at location I
                for i in 0..=x {
                    self.registers[i as usize] =
                        self.memory[self.index_register as usize + i as usize];
                }

                // 如果你想兼容极少数老 ROM，可以取消下面这行的注释
                // self.index += x + 1;
            }

            (0xF, _, 0x5, 0x5) => {
                // FX55: Store registers V0 through Vx in memory starting at location I
                for i in 0..=x {
                    self.memory[self.index_register as usize + i as usize] =
                        self.registers[i as usize];
                }

                // 同样，为了兼容现代 ROM，通常不增加 I
                // self.index += x + 1;
            }

            (0xF, _, 0x3, 0x3) => {
                // FX33: Store BCD representation of Vx in memory locations I, I+1, and I+2
                let value = self.registers[x as usize];

                // 百位
                self.memory[self.index_register as usize] = value / 100;
                // 十位
                self.memory[self.index_register as usize + 1] = (value / 10) % 10;
                // 个位
                self.memory[self.index_register as usize + 2] = value % 10;
            }
            (0xF, _, 0x1, 0xE) => {
                // FX1E: Set I = I + Vx
                let vx_value = self.registers[x as usize] as u16;

                // 传统的加法，通常不考虑溢出设置 VF (但某些 Quirk 会要求设置)
                self.index_register = self.index_register.wrapping_add(vx_value);
            }
            (0x8, _, _, 0x7) => {
                // 8XY7: Vx = Vy - Vx, set VF = NOT borrow
                let vx = self.registers[x as usize];
                let vy = self.registers[y as usize];

                // 注意这里是 vy 减去 vx
                let (diff, did_borrow) = vy.overflowing_sub(vx);

                self.registers[x as usize] = diff;

                // 同样是“没借位为 1”，即 vy >= vx 时 VF 为 1
                self.registers[0xF] = if !did_borrow { 1 } else { 0 };
            }
            (0xA, _, _, _) => {
                // ANNN: 加载 NNN 到 I 寄存器
                self.index_register = nnn;
            }
            (0x7, _, _, _) => {
                // 7XNN: Vx += NN
                // 注意：x 是寄存器索引，nn 是提取出的低 8 位
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(nn);
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
