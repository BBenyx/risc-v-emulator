use crate::instruction::*;

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub memory: Vec<u8>,
}

impl Cpu {
    fn fetch(&self) -> u32 {
        let mem = &self.memory;
        let pc = self.pc;

        let instruction: [u8; 4] =
        [mem[pc as usize],
        mem[(pc+1) as usize],
        mem[(pc+2) as usize],
        mem[(pc+3) as usize]];
        u32::from_le_bytes(instruction)
    }

    fn decode(instr: u32) -> Instruction {
        let opcode = instr & 0x7f;

        match opcode {
            0x33 /* R-type 32bit*/ => Instruction::RType(RTypeInstr::parse(instr)),
            0x3b /* R-type 64bit */ => Instruction::RType(RTypeInstr::parse(instr)),
            0x13 /* I-type ALU */ => Instruction::IType(ITypeInstr::parse(instr)),
            0x03 /* I-type Load */ => Instruction::IType(ITypeInstr::parse(instr)),
            0x23 /* S-type */ => Instruction::SType(STypeInstr::parse(instr)),
            0x63 /* B-type */ => Instruction::BType(BTypeInstr::parse(instr)),
            0x6f /* J-type */ => Instruction::JType(JTypeInstr::parse(instr)),
            // 0x37 /* U-Type */ can be implemented here
            _ => panic!("Unimplemented opcode: {opcode}"),
        }
    }

    fn execute(&mut self, instr: Instruction) {
        instr.execute(&mut self.regs, &mut self.pc);
    }

    pub fn run(&mut self) {
        while (self.pc as usize) < self.memory.len() {
            let bytes = self.fetch();

            let decoded = Self::decode(bytes);
            //println!("{:?}", &decoded);
            self.execute(decoded);

            //println!("PC: {}", self.pc);
            //println!("Reg 6: {}", self.regs[6]);

            self.pc += 4;
        }

        println!("Registers:");
        for i in 0..16 {
            println!("{i1: <2}: {column1: <15}{i2: >15}: {column2}",
            i1 = i, i2 = i+16,
            column1 = self.regs[i], column2 = self.regs[i+16]);
        }
    }
}

pub fn u32_from_bits(bits: &[u8]) -> u32 {
    let mut value: u32 = 0;

    for &bit in bits {
        value = (value << 1) | (bit as u32);
    }

    value
}