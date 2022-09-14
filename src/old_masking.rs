fn decode_rio(inst: u16) -> String { // Integer Register-Immediate Operation
    let mut dis_inst = String::new(); // disassembled instruction
    
    // funct
    dis_inst.push_str(get_funct_rio((inst & 0b1110000000000000) >> 13).as_str());
    dis_inst.push_str(" ");

    // rd
    dis_inst.push_str(get_reg((inst & 0b0000111110000000) >> 7).as_str()); // same
    dis_inst.push_str(",");
    dis_inst.push_str(get_reg((inst & 0b0000111110000000) >> 7).as_str()); // same

    // imm
    dis_inst.push_str(",");
    let mut imm = ((inst & 0b0000000001111100) >> 2) as i32;
    if (inst & 0b0001000000000000) != 0{
        imm = -imm;
    }
    dis_inst.push_str((imm).to_string().as_str());

    dis_inst
    /*
    0000011110000101
    &   0000000001111100
    =   0000000000000100
    */
}

fn get_reg(inst: u16) -> String { // change return val to &str
    let abi = ["zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"];
    if inst > 0b100000 { // 32
        panic!("Invalid register number: 0b{:b}", inst);
    } else {
        return abi[inst as usize].to_string();
    }
}

fn get_funct_rio(inst: u16) -> String {
    let mut funct = String::new();
    match inst {
        0b000 => funct.push_str("addi"),
        0b001 => funct.push_str("addiw"),
        _ => panic!("Invalid funct: 0b{:b}", inst),
    }
    funct
}

fn two_complement(num: u16) -> i32 {
    let inverse: i32 = num ^ 0b111111111111;
    inverse += 1;

    inverse = -inverse;
}

fn get_funct_store(inst: u32) -> String {
    let funct = ["sb", "sh", "sw", "sd"];
    if inst > 0b100 { // 4
        panic!("Invalid register number: 0b{:b}", inst);
    } else {
        return funct[inst as usize].to_string();
    }
}

fn decode_store(inst: u32) -> String {
    let mut dis_inst = String::new(); // disassembled instruction

    // funct
    dis_inst.push_str(get_funct_store((inst & 0b0000000000000001110000000000000) >> 14).as_str());
    dis_inst.push_str(" ");

    // rs2
    dis_inst.push_str(get_reg((inst & 0b0000000111110000000000000000000) >> 19).as_str());
    dis_inst.push_str(",");

    // imm
    let mut imm = (inst & 0b1111111000000000000000000000000) >> 20;
    imm += (inst & 0b0000000000000000000111110000000) >> 7;
    if imm > 0b1111111111111111 {
        imm = two_complement(imm);
    } else {
        imm = imm as i32;
    }
    dis_inst.push_str((imm).to_string().as_str());
    dis_inst.push_str("(");

    // rs1
    dis_inst.push_str(get_reg((inst & 0b0000000000011111000000000000000) >> 14).as_str());
    dis_inst.push_str(")");

    dis_inst
}

fn hello() {
    println!("{}", decode_rio(0x078d));     // addi a5,a5,01
    println!("{}", decode_rio(0x0789));     // addi a5,a5,02
    println!("{}", decode_rio(0x057d));     // addi a0,a0,31
    println!("{}", decode_rio(0x1141));     // addi sp,sp,-16
    println!("{}", decode_rio(0x0400));     // addi sp,sp,-16
    //println!("{}", decode_rio(0xfea42623)); // sw a0,-20(s0)
}

/*
C.ADDI
num + 01 addi a5,a5,01 0785 000 0 01111 00001 01
num + 02 addi a5,a5,02 0789 000 0 01111 00010 01
num + 03 addi a5,a5,03 078d 000 0 01111 00011 01
num + 04 addi a5,a5,04 0791 000 0 01111 00100 01
num + 05 addi a5,a5,05 0795 000 0 01111 00101 01
*/

/*
SW
sw a0,-20(s0) fea42623 1111111 01010 01000 010 01100 0100011
                       5432198 57321 19876 543 21198 7654321    
*/