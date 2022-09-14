use bitfield::bitfield;

bitfield! {
    pub struct C_RIO(u16);
    impl Debug;
    pub funct, get_funct: 0b1110000000000000;
    pub rd, get_rd: 0b0000111110000000;
    pub imm, get_imm: 0b0000000001111100;
    pub neg, get_neg: 0b0001000000000000;
}

fn decode_c_rio(inst: u16) -> String {
    let inst = C_RIO(inst);
    let funct = match inst.funct() {
        0b000 => "addi",
        0b001 => "addiw",
        _ => panic!("Invalid funct: 0b{:b}", inst.funct()),
    };

    let rd = {
        if inst.rd() < 0b10000 {
            let mut abi = ["zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"];
            abi[inst.rd() as usize].to_string()
        } else {
            panic!("Invalid register number: 0b{:b}", inst.rd());
        }
    };

    let imm = {
        let mut imm = inst.imm() as i32;
        if inst.neg() {
            imm = -imm;
        }
        imm
    };

    format!("{} {},{}, {}", funct, rd, rd, imm)
}

fn main() {
    println!("{}", decode_c_rio(0x078d));     // addi a5,a5,01
    println!("{}", decode_c_rio(0x0789));     // addi a5,a5,02
    println!("{}", decode_c_rio(0x057d));     // addi a0,a0,31
    println!("{}", decode_c_rio(0x1141));     // addi sp,sp,-16
    println!("{}", decode_c_rio(0x0400));     // addi sp,sp,-16
}