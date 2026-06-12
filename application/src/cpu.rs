use crate::instruction::{self, BTypeInstr, ITypeInstr, JTypeInstr, RTypeInstr, STypeInstr};

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: usize,
    pub memory: Vec<u8>,
}

impl Cpu {
    fn fetch(&self) -> u32 {
        u32_from_bits(&self.memory[self.pc..self.pc + 32])
    }

    fn decode(instr: u32) -> instruction::Instruction {
        let opcode = instr & 0x7f;
        println!("{:x}", opcode);

        match opcode {
            0x33 /* R-type 32bit*/ => instruction::Instruction::RType(RTypeInstr::parse(instr)),
            0x3B /* R-type 64bit */ => instruction::Instruction::RType(RTypeInstr::parse(instr)),
            0x13 /* I-type ALU */ => instruction::Instruction::IType(ITypeInstr::parse(instr)),
            0x03 /* I-type Load */ => instruction::Instruction::IType(ITypeInstr::parse(instr)),
            0x23 /* S-type */ => instruction::Instruction::SType(STypeInstr::parse(instr)),
            0x63 /* B-type */ => instruction::Instruction::BType(BTypeInstr::parse(instr)),
            0x6f /* J-type */ => instruction::Instruction::JType(JTypeInstr::parse(instr)),
            // 0x37 /* U-Type */ can be implemented here
            _ => panic!("Unimplemented opcode: {opcode}"),
        }
    }

    fn execute(&mut self, instr: instruction::Instruction) {
        todo!()
    }

    pub fn run(&mut self) {
        while self.pc <= self.memory.len() {
            let bytes = self.fetch();

            let decoded = Self::decode(bytes);
            //self.execute(decoded);
            println!("{:?}", decoded);

            self.pc += 32;
        }

        println!("Registers:");
        for (i, elem) in self.memory.iter().enumerate() {
            println!("{i}: {elem}");
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