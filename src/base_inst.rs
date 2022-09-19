pub fn base_inst_from(inst: u32) -> String {
    let opcode = inst & 0b1111111;
    
    let line = match opcode {
        0b0000011 => {
            let mut i = I {
                funct3: 0,
                rd: 0,
                rs1: 0,
                imm: 0,
            };
            i.decode(inst);
            i.format()
        }
        _ => String::new(),
    };
    line
}

struct R {
    funct3: u32,
    rd: u32,
    rs1: u32,
    rs2: u32,
    funct7: u32,
}

impl R {
    fn decode(&mut self, inst: u32) {
        self.rd = (inst & 0b00000000000000000000011111000000) >> 6;
        self.funct3 = (inst & 0b00000000000000000011100000000000) >> 11;
        self.rs1 = (inst & 0b00000000000001111100000000000000) >> 14;
        self.rs2 = (inst & 0b00000000111110000000000000000000) >> 19;
        self.funct7 = (inst & 0b11111111000000000000000000000000) >> 24;
    }
}

struct I {
    funct3: u32,
    rd: u32,
    rs1: u32,
    imm: u32,
}

impl I {
    fn decode(&mut self, inst: u32) {
        self.rd = (inst & 0b00000000000000000000111110000000) >> 7;
        self.funct3 = (inst & 0b00000000000000000111000000000000) >> 12;
        self.rs1 = (inst & 0b00000000000011111000000000000000) >> 15;
        self.imm = (inst & 0b11111111111100000000000000000000) >> 20;
    }

    fn format(&self) -> String {
        let funct = match self.funct3 {
            0b000 => "lb",
            0b001 => "lh",
            0b010 => "lw",
            0b100 => "lbu",
            0b101 => "lhu",
            _ => return String::new(),
        };

        let rd = get_reg(self.rd);
        let rs1 = get_reg(self.rs1);
        let imm = {
            if self.imm & 0b100000000000 != 0 {
                -((!self.imm & 0b111111111111) as i32 + 1)
            } else {
                self.imm as i32
            }
        };
        
        if rd == String::new() || rd == String::new() {
            return String::new();
        } else {
            format!("{} {},{}({})", funct, rd, imm, rs1)
        }
    }
}

struct S {
    funct3: u32,
    rs1: u32,
    rs2: u32,
    imm: u32,
}

impl S {
    fn decode(&mut self, inst: u32) {
        self.funct3 = (inst & 0b00000000000000000011100000000000) >> 11;
        self.rs1 = (inst & 0b00000000000001111100000000000000) >> 14;
        self.rs2 = (inst & 0b00000000111110000000000000000000) >> 19;

        let imm1 = (inst & 0b00000000000000000000011111000000) >> 6;
        self.imm = ((inst & 0b11111111000000000000000000000000) >> 18) + imm1;
    }
}

struct U {
    rd: u32,
    imm: u32,
}

impl U {
    fn decode(&mut self, inst: u32) {
        self.rd = (inst & 0b00000000000000000000011111000000) >> 6;
        self.imm = (inst & 0b11111111111111111111100000000000) >> 11;
    }
}

struct B {
    funct3: u32,
    rs1: u32,
    rs2: u32,
    imm: u32,
}

struct J {
    rd: u32,
    imm: u32,
}

fn get_reg(inst: u32) -> String {
    if inst < 0b10000 {
        let abi = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
            "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
            "t3", "t4", "t5", "t6",
        ];
        abi[inst as usize].to_string()
    } else {
        return String::new();
    }
}