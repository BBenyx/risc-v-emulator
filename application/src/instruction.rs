
pub enum Instruction {
    RType(RTypeInstr),
    IType(ITypeInstr),
    SType(STypeInstr),
    BType(BTypeInstr),
    UType(UTypeInstr),
    JType(JTypeInstr),
}


pub struct RTypeInstr {
    opcode: u8,
    rd: u8,
    rs1: u8,
    rs2: u8,
    funct3: u8,
    funct7: u8,
}

impl RTypeInstr {
    pub fn parse(instr: &[u8]) -> Self {
        Self {
            opcode: u8_from_bits(&instr[0..6]),
            rd: u8_from_bits(&instr[7..11]),
            funct3: u8_from_bits(&instr[12..14]),
            rs1: u8_from_bits(&instr[15..19]),
            rs2: u8_from_bits(&instr[20..24]),
            funct7: u8_from_bits(&instr[25..31]),
        }
    }
}


pub struct ITypeInstr {
    opcode: u8,
    rd: u8,
    rs1: u8,
    funct3: u8,
    imm: u8,
}

impl ITypeInstr {
    pub fn parse(instr: &[u8]) -> Self {
        Self {
            opcode: u8_from_bits(&instr[0..6]),
            rd: u8_from_bits(&instr[7..11]),
            funct3: u8_from_bits(&instr[12..14]),
            rs1: u8_from_bits(&instr[15..19]),
            imm: u8_from_bits(&instr[20..31]),
        }
    }
}


pub struct STypeInstr {
    opcode: u8,
    rs1: u8,
    rs2: u8,
    funct3: u8,
    imm: u8,
}

impl STypeInstr {
    pub fn parse(instr: &[u8]) -> Self {
        let mut imm_vec: Vec<u8> = Vec::new();
        imm_vec.extend_from_slice(&instr[7..11]);
        imm_vec.extend_from_slice(&instr[25..31]);

        Self {
            opcode: u8_from_bits(&instr[0..6]),
            imm: u8_from_bits(&imm_vec),
            funct3: u8_from_bits(&instr[12..14]),
            rs1: u8_from_bits(&instr[15..19]),
            rs2: u8_from_bits(&instr[20..24]),
        }
    }
}


pub struct BTypeInstr {
    opcode: u8,
    rs1: u8,
    rs2: u8,
    funct3: u8,
    imm: u8,
}

impl BTypeInstr {
    pub fn parse(instr: &[u8]) -> Self {
        let mut imm_vec: Vec<u8> = Vec::new();
        imm_vec.extend_from_slice(&instr[8..11]);
        imm_vec.extend_from_slice(&instr[25..30]);
        imm_vec.push(instr[7]);
        imm_vec.push(instr[31]);


        Self {
            opcode: u8_from_bits(&instr[0..6]),
            imm: u8_from_bits(&imm_vec),
            funct3: u8_from_bits(&instr[12..14]),
            rs1: u8_from_bits(&instr[15..19]),
            rs2: u8_from_bits(&instr[20..24]),
        }
    }
}


pub struct UTypeInstr {
    opcode: u8,
    rd: u8,
    imm: u8,
}

impl UTypeInstr {
    pub fn parse(instr: &[u8]) -> Self {
        Self {
            opcode: u8_from_bits(&instr[0..6]),
            rd: u8_from_bits(&instr[7..11]),
            imm: u8_from_bits(&instr[12..31]),
        }
    }
}


pub struct JTypeInstr {
    opcode: u8,
    rd: u8,
    imm: u8,
}

impl JTypeInstr {
    pub fn parse(instr: &[u8]) -> Self {
        let mut imm_vec: Vec<u8> = Vec::new();
        imm_vec.extend_from_slice(&instr[21..30]);
        imm_vec.push(instr[20]);
        imm_vec.extend_from_slice(&instr[12..19]);
        imm_vec.push(instr[31]);


        Self {
            opcode: u8_from_bits(&instr[0..6]),
            rd: u8_from_bits(&instr[7..11]),
            imm: u8_from_bits(&imm_vec),
        }
    }
}




pub fn u8_from_bits(slice: &[u8]) -> u8 {
    let mut value: u8 = 0;

    for &bit in slice {
        value = (value << 1) | bit;
    }

    value
}