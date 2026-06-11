use crate::instruction::{self, BTypeInstr, ITypeInstr, JTypeInstr, RTypeInstr, STypeInstr};

pub struct Cpu {
    pub regs: [u64; 64],
    pub pc: usize,
    pub memory: Vec<u8>,
}

impl Cpu {
    fn fetch(&self) -> &[u8] {
        &self.memory[self.pc..self.pc + 32]
    }

    fn decode(instr: &[u8]) -> instruction::Instruction {
        let opcode = instruction::u8_from_bits(&instr[0..6]);

        match opcode {
            51 /* R-type */ => instruction::Instruction::RType(RTypeInstr::parse(instr)),
            19 /* I-type */ => instruction::Instruction::IType(ITypeInstr::parse(instr)),
            3 /* I-type */ => instruction::Instruction::IType(ITypeInstr::parse(instr)),
            35 /* S-type */ => instruction::Instruction::SType(STypeInstr::parse(instr)),
            99 /* B-type */ => instruction::Instruction::BType(BTypeInstr::parse(instr)),
            111 /* J-type */ => instruction::Instruction::JType(JTypeInstr::parse(instr)),
            103 /* I-type */ => instruction::Instruction::IType(ITypeInstr::parse(instr)),
            _ => panic!("Unimplemented opcode: {opcode}"),
        }
    }

    fn execute(&mut self, instr: instruction::Instruction) {
        todo!()
    }

    fn run(&mut self) {
        while self.pc <= self.memory.len() {
            let bytes = self.fetch();

            let decoded = Self::decode(bytes);
            self.execute(decoded);

            self.pc += 32;
        }

        println!("Registers:");
        for (i, elem) in self.memory.iter().enumerate() {
            println!("{i}: {elem}");
        }
    }
}