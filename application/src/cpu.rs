use crate::instruction::*;

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: usize,
    pub memory: Vec<u8>,
}

impl Cpu {
    fn fetch(&self) -> u32 {
        u32_from_bits(&self.memory[self.pc..self.pc + 32])
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

    // sziabeniszeretlek
    fn execute(&mut self, instr: Instruction) {
        instr.execute(&mut self.regs);
    }

    pub fn run(&mut self) {
        while self.pc < self.memory.len() {
            let bytes = self.fetch();

            let decoded = Self::decode(bytes);
            println!("{:?}", &decoded);
            self.execute(decoded);

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