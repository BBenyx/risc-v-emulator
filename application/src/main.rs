use std::env::args;

mod instruction;
mod file_reader;

#[derive(Debug)]
enum MainError {
    NoArgumentError,
    NoFileFoundOnPathError,
}

struct Cpu {
    regs: [u64; 64],
    pc: u64,
    memory: Vec<u8>,
}

impl Cpu {
    fn fetch(&self) -> u32 {
        todo!()
    }

    fn decode(instr: u32) -> instruction::Instruction {
        todo!()
    }

    fn execute(&mut self, instr: instruction::Instruction) {
        todo!()
    }

    fn run(&mut self) {
        todo!()
    }
}

fn main() -> Result<(), MainError> {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        return Err(MainError::NoArgumentError);
    }

    let cpu: Cpu = Cpu {
        regs: [0; 64],
        pc: 0,
        memory: {
            match file_reader::read_file_bits(&args[1]) {
                Some(v) => v,
                None => return Err(MainError::NoFileFoundOnPathError),
            }
        },
    };


    println!("{:?}", cpu.memory);

    Ok(())
}
