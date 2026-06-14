#[derive(Debug)]
pub enum Instruction {
    RType(RTypeInstr),
    IType(ITypeInstr),
    SType(STypeInstr),
    BType(BTypeInstr),
    //UType(UTypeInstr),
    JType(JTypeInstr),
}

impl Instruction {
    pub fn execute(&self, regs: &mut [u64; 32], pc: &mut u64) {
        match self {
            Instruction::RType(instr) => instr.execute(regs),
            Instruction::IType(instr) => instr.execute(regs),
            Instruction::SType(instr) => todo!(),
            Instruction::BType(instr) => instr.execute(regs, pc),
            Instruction::JType(instr) => instr.execute(regs, pc),
        }
    }
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

    pub fn execute(&self, regs: &mut [u64; 32]) {
        match self.opcode {
            0x33 /* 32bit */=> self.b32_operation(regs),
            0x3b /* 64bit */ => self.b64_operation(regs),
            _ => panic!("Undefined I-Type opcode: {:x}", self.opcode),
        }
    }

    fn b64_operation(&self, regs: &mut [u64; 32]) {
        match self.funct3 {
            0x0 /* MULW */=> {
                // Multiplication is done by sign extension
                regs[self.rd as usize] =
                    ((regs[self.rs1 as usize] as i32).
                    wrapping_mul(regs[self.rs2 as usize] as i32)) as i64 as u64
            },
            _ => panic!("Undefined funct3 value: {:x}", self.funct3),
        }
    }

    fn b32_operation(&self, regs: &mut [u64; 32]) {
        match (self.funct3, self.funct7) {
            (0x0, 0x0) /* ADD */ => {
                regs[self.rd as usize] =
                    regs[self.rs1 as usize].
                    wrapping_add(regs[self.rs2 as usize]);
            },
            _ => panic!("Undefined funct3-7 combo value: ({:x},{:x})",
            self.funct3, self.funct7),
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
        let imm = ((instr >> 20) & 0xfff) as i32;

        Self {
            opcode: (instr & 0x7f) as u8,
            rd: ((instr >> 7) & 0x1f) as u8,
            funct3: ((instr >> 12) & 0x7) as u8,
            rs1: ((instr >> 15) & 0x1f) as u8,
            imm: (imm << 20) >> 20, // Sign extend (see in sources.txt)
        }
    }

    pub fn execute(&self, regs: &mut [u64; 32]) {
        match self.opcode {
            0x13 /* ALU */=> self.alu_operation(regs),
            0x03 => todo!("Load operation"),
            _ => panic!("Undefined I-Type opcode: {:x}", self.opcode),
        }
    }

    fn alu_operation(&self, regs: &mut [u64; 32]) {
        match self.funct3 {
            0x0 /* ADDI */=> {
                regs[self.rd as usize] =
                    (regs[self.rs1 as usize]).
                    wrapping_add(self.imm as u64)   
            },
            _ => panic!("Undefined funct3 value: {:x}", self.funct3),
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

        let imm = ((imm_11_5 << 5) | imm_4_0) as i32;

        Self {
            funct3: ((instr >> 12) & 0x7) as u8,
            rs1: ((instr >> 15) & 0x1f) as u8,
            rs2: ((instr >> 20) & 0x1f) as u8,
            imm: (imm << 20) >> 20, // Sign extend (see in sources.txt)
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
        let imm_11 = (instr >> 7) & 0x1;
        let imm_4_1 = (instr >> 8) & 0xf;
        let imm_10_5 = (instr >> 25) & 0x3f;
        let imm_12 = (instr >> 31) & 0x1;

        let imm = ((imm_12 << 12)
                | (imm_11 << 11)
                | (imm_10_5 << 5)
                | (imm_4_1 << 1)) as i32;

        Self {
            funct3: ((instr >> 12) & 0x7) as u8,
            rs1: ((instr >> 15) & 0x1f) as u8,
            rs2: ((instr >> 20) & 0x1f) as u8,
            imm: (imm << 19) >> 19, // Sign extend (see in sources.txt)
        }
    }

    pub fn execute(&self, regs: &mut [u64; 32], pc: &mut u64) {
        match self.funct3 {
            0x0 /* BEQ */ => {
                if regs[self.rs1 as usize] == regs[self.rs2 as usize] {
                    *pc = pc.wrapping_add_signed(self.imm as i64 - 4);
                }
            },
            _ => panic!("Undefined funct3 value: {:x}", self.funct3),
        }
    }
}

/*
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
*/

#[derive(Debug)]
pub struct JTypeInstr {
    rd: u8,
    imm: i32,
}

impl JTypeInstr {
    pub fn parse(instr: u32) -> Self {
        let imm_20 = (instr >> 31) & 0x1;
        let imm_10_1 = (instr >> 21) & 0x3ff;
        let imm_11 = (instr >> 20) & 0x1;
        let imm_19_12 = (instr >> 12) & 0xff;

    let imm = ((imm_20 << 20)
            | (imm_19_12 << 12)
            | (imm_11 << 11)
            | (imm_10_1 << 1)) as i32;

    let imm = ((imm << 11) as i32) >> 11;  

        Self {
            rd: ((instr >> 7) & 0x1f) as u8,
            imm: (imm << 11) >> 11, // Sign extend (see in sources.txt)
        }
    }

    pub fn execute(&self, regs: &mut [u64; 32], pc: &mut u64) {
        let next = pc.wrapping_add(4);

        if self.rd != 0 {
            regs[self.rd as usize] = next;
        }
        // - 4 needed beacuse the pc will be increased by 4 after the jump instruction!
        *pc = pc.wrapping_add_signed(self.imm as i64 - 4);
    }
}