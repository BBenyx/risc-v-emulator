use crate::instruction::*;

pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,

    pub memory: Vec<u8>,

    pub text_end: u64,
    pub stack_start: u64,
}

impl Cpu {
    pub fn new(program: Vec<u8>) -> Self {
        // 0.5 KB
        let mut memory: Vec<u8> = vec![0; 512];
        for (i, byte) in program.iter().enumerate() {
            memory[i] = *byte;
        }

        let mut cpu = Cpu {
            regs: [0; 32],
            pc: 0,
            memory,

            text_end: program.len() as u64,
            stack_start: program.len() as u64,
        };

        // stack pointer
        cpu.regs[2] = cpu.stack_start + 128;

        cpu
    }

    fn fetch(&self) -> u32 {
        let mem = &self.memory;
        let pc = self.pc as usize;
        

        let instruction: [u8; 4] =
        [mem[pc],
        mem[pc + 1],
        mem[pc + 2],
        mem[pc + 3]];
        
        u32::from_le_bytes(instruction)
    }

    fn decode(instr: u32) -> Instruction {
        let opcode = instr & 0x7f;
        println!("{:b}", &opcode);

        match opcode {
            0x33 /* R-type 32bit*/ => Instruction::RType(RTypeInstr::parse(instr)),
            0x3b /* R-type 64bit */ => Instruction::RType(RTypeInstr::parse(instr)),
            0x13 /* I-type ALU */ => Instruction::IType(ITypeInstr::parse(instr)),
            0x03 /* I-type Load */ => Instruction::IType(ITypeInstr::parse(instr)),
            0x67 /* I-type return */ => Instruction::IType(ITypeInstr::parse(instr)),
            0x23 /* S-type */ => Instruction::SType(STypeInstr::parse(instr)),
            0x63 /* B-type */ => Instruction::BType(BTypeInstr::parse(instr)),
            0x6f /* J-type */ => Instruction::JType(JTypeInstr::parse(instr)),
            // 0x37 /* U-Type */ can be implemented here
            _ => panic!("Unimplemented opcode: {opcode}"),
        }
    }

    fn execute(&mut self, instr: Instruction) {
        instr.execute(self);
    }

    pub fn run(&mut self) {
        while (self.pc as usize) < self.text_end as usize {
            let bytes = self.fetch();

            let decoded = Self::decode(bytes);
           //println!("{:?}", &decoded);
            self.execute(decoded);

            //println!("PC: {}", self.pc);

            self.pc += 4;
        }

        println!("Registers:");
        for i in 0..16 {
            println!("{i1: <2}: {column1: <15}{i2: >15}: {column2}",
            i1 = i, i2 = i+16,
            column1 = self.regs[i], column2 = self.regs[i+16]);
        }
        println!("Full memory: {:?}", self.memory);
    }
}