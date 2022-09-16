fn rvc_from(inst: u16) -> String {
    let opcode = inst & 0b11;

    let line = match opcode {
        0b01 => {
            let mut c_rio = C1 {
                funct: 0,
                rd: 0,
                imm: 0,
                neg: 0,
            };
            c_rio.decode(inst);
            c_rio.format()
        }
        0b10 => {
            let mut c_store = C2 {
                funct: 0,
                rs2: 0,
                rs1: 0,
                flag: 0,
            };
            c_store.decode(inst);
            c_store.format()
        }
        _ => return String::new(),
    };
    line
}

struct C1 {
    funct: u16,
    rd: u16,
    imm: u16,
    neg: u16,
}

impl C1 {
    fn decode(&mut self, inst: u16) {
        self.funct = (inst & 0b1110000000000000) >> 13;
        self.rd = (inst & 0b0000111110000000) >> 7;
        self.imm = (inst & 0b0000000001111100) >> 2;
        self.neg = inst & 0b0001000000000000;
    }

    fn format(&self) -> String {
        let funct = match self.funct {
            0b000 => "addi",
            0b001 => "addiw",
            0b010 => "li",
            _ => return String::new(),
        };

        let rd = get_reg(self.rd);
        if rd == "zero" && self.funct == 0b000 {
            return "nop".to_string();
        }

        let imm = {
            let mut imm = self.imm as i32;
            if self.neg != 0 {
                imm = -imm;
            }
            imm
        };

        return match funct {
            "li" => format!("{} {},{}", funct, rd, imm),
            _ => format!("{} {},{},{}", funct, rd, rd, imm),
        };
    }
}

struct C2 {
    funct: u16,
    rs2: u16,
    rs1: u16,
    flag: u16, // used to find imm or determine funct
}

impl C2 {
    fn decode(&mut self, inst: u16) {
        self.funct = (inst & 0b1110000000000000) >> 13;
        self.rs2 = (inst & 0b0000000001111100) >> 2;
        self.rs1 = (inst & 0b0000111110000000) >> 7;
        self.flag = (inst & 0b0001000000000000) >> 12;
    }

    fn format(&self) -> String {
        let funct = match self.funct {
            0b100 => "jr",
            _ => return String::new(),
        };

        let rs1 = get_reg(self.rs1);
        let rs2 = get_reg(self.rs2);
        
        match funct {
            "jr" => {
                if self.flag == 0 && self.rs2 == 0 {
                    return format!("{} {}", funct, rs1);
                } else if self.flag == 0 && self.rs2 != 0 {
                    return format!("mv {},{}", rs1, rs2);
                } else if self.flag == 1 && self.rs1 == 0 && self.rs2 == 0 {
                    return "ebreak".to_string();
                } else if  self.flag == 1 && self.rs2 == 0 {
                    return format!("jalr {}", rs1);
                } else if self.flag == 1 && self.rs1 != 0 && self.rs2 != 0{
                    return format!("add {},{},{}", rs1, rs1, rs2);
                } else {
                    return String::new();
                }
            }
            _ => return String::new(),
        };
    }
}

fn get_reg(inst: u16) -> String {
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

fn main() {
    let inst = 0x853e;
    let line = rvc_from(inst);

    if line == String::new() {
        println!("{}", inst);
    } else {
        println!("{}", line);
    }
}
