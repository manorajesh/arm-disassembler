pub fn base_inst_from(inst: u32) -> String {
    format!("db {:b}", inst)
}

struct R {
    funct3: u32,
    rd: u32,
    rs1: u32,
    rs2: u32,
    funct7: u32,
}

impl R {
    fn decode (&mut self, inst: u32) {
        self.rd =       (inst & 0b00000000000000000000011111000000) >> 6; // wrong
        self.funct3 =   (inst & 0b00000000000000000001110000000000) >> 7; // wrong
        self.rs1 =      (inst & 0b00000000000000111110000000000000) >> 15; // wrong
        self.rs2 =      (inst & 0b00000000011111000000000000000000) >> 20; // wrong
        self.funct7 =   (inst & 0b11111111100000000000000000000000) >> 25; // wrong
    }
}

struct I {
    funct3: u32,
    rd: u32,
    rs1: u32,
    imm: u32,
}

impl I {
    fn decode (&mut self, inst: u32) {
        self.funct3 = (inst & 0b00000000000000000000000001110000) >> 12;
        self.rd = (inst & 0b00000000000000000000111110000000) >> 7;
        self.rs1 = (inst & 0b00000000000001111110000000000000) >> 15;
        self.imm = (inst & 0b11111111111110000000000000000000) >> 20;
    }
}

struct S {
    funct3: u32,
    rs1: u32,
    rs2: u32,
    imm: u32,
}

struct B {
    funct3: u32,
    rs1: u32,
    rs2: u32,
    imm: u32,
}

struct U {
    rd: u32,
    imm: u32,
}

struct J {
    rd: u32,
    imm: u32,
}