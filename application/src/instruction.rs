#[derive(Debug)]
pub enum Instruction {
    RType(RTypeInstr),
    IType(ITypeInstr),
    SType(STypeInstr),
    BType(BTypeInstr),
    UType(UTypeInstr),
    JType(JTypeInstr),
}

#[derive(Debug)]
pub struct RTypeInstr {
    opcode: u8,
    rd: u8,
    rs1: u8,
    rs2: u8,
    funct3: u8,
    funct7: u8,
}

impl RTypeInstr {
    pub fn parse(instr: u32) -> Self {
        Self {
            opcode: (instr & 0x7f) as u8,
            rd: ((instr >> 7) & 0x1f) as u8,
            funct3: ((instr >> 12) & 0x7) as u8,
            rs1: ((instr >> 15) & 0x1f) as u8,
            rs2: ((instr >> 20) & 0x1f) as u8,
            funct7: ((instr >> 25) & 0x7f) as u8,
        }
    }
}

#[derive(Debug)]
pub struct ITypeInstr {
    opcode: u8,
    rd: u8,
    rs1: u8,
    funct3: u8,
    imm: i32,
}

impl ITypeInstr {
    pub fn parse(instr: u32) -> Self {
        Self {
            opcode: (instr & 0x7f) as u8,
            rd: ((instr >> 7) & 0x1f) as u8,
            funct3: ((instr >> 12) & 0x7) as u8,
            rs1: ((instr >> 15) & 0x1f) as u8,
            imm: ((instr >> 20) & 0x3ff) as i32,
        }
    }
}

#[derive(Debug)]
pub struct STypeInstr {
    rs1: u8,
    rs2: u8,
    funct3: u8,
    imm: i32,
}

impl STypeInstr {
    pub fn parse(instr: u32) -> Self {
        let imm_4_0 = (instr >> 7) & 0x1f;
        let imm_11_5 = (instr >> 25) & 0x7f;

        Self {
            imm: ((imm_11_5 << 5) | imm_4_0) as i32,
            funct3: ((instr >> 12) & 0x7) as u8,
            rs1: ((instr >> 15) & 0x1f) as u8,
            rs2: ((instr >> 20) & 0x1f) as u8,
        }
    }
}

#[derive(Debug)]
pub struct BTypeInstr {
    rs1: u8,
    rs2: u8,
    funct3: u8,
    imm: i32,
}

impl BTypeInstr {
    pub fn parse(instr: u32) -> Self {
        let imm_4_1 = (instr >> 8) & 0xf;
        let imm_10_5 = (instr >> 25) & 0x3f;
        let imm_11 = (instr >> 7) & 0x1;
        let imm_12 = (instr >> 31) & 0x1;

        Self {
            imm: ((imm_12 << 12) | (imm_11 << 11) | (imm_10_5 << 5) | (imm_4_1 << 1)) as i32,
            funct3: ((instr >> 12) & 0x7) as u8,
            rs1: ((instr >> 15) & 0x1f) as u8,
            rs2: ((instr >> 20) & 0x1f) as u8,
        }
    }
}

#[derive(Debug)]
pub struct UTypeInstr {
    rd: u8,
    imm: u32,
}

impl UTypeInstr {
    pub fn parse(instr: u32) -> Self {
        Self {
            rd: ((instr >> 7) & 0x1f) as u8,
            imm: ((instr >> 12) & 0x3ff) as u32,
        }
    }
}

#[derive(Debug)]
pub struct JTypeInstr {
    rd: u8,
    imm: i32,
}

impl JTypeInstr {
    pub fn parse(instr: u32) -> Self {
        let imm_10_1 = (instr >> 21) & 0x3ff;
        let imm_117 = (instr >> 20) & 0x1;
        let imm_19_12 = (instr >> 12) & 0xff;
        let imm_20 = (instr >> 31) & 0x1;

        Self {
            rd: ((instr >> 7) & 0x1f) as u8,
            imm: ((imm_20 << 20) | (imm_19_12 << 12) | (imm_117 << 11) | (imm_10_1 << 1)) as i32,
        }
    }
}
