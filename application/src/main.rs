use std::env::args;
use std::fs;

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

    /*fn decode(instr: u32) -> Instruction {
        todo!()
    }

    fn execute(&mut self, instr: Instruction) {
        todo!()
    }

    fn run(&mut self) {
        todo!()
    }*/
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
            match read_file_bits(&args[1]) {
                Some(v) => v,
                None => return Err(MainError::NoFileFoundOnPathError),
            }
        },
    };


    println!("{:?}", cpu.memory);

    Ok(())
}

fn read_file_bits(path: &str) -> Option<Vec<u8>> {
    let bytes = fs::read(path).ok()?;
    let mut bits: Vec<u8> = Vec::new();

    for chunk in bytes.chunks(4) {
        if chunk.len() > 4 { break; }

        let value = u32::from_le_bytes([
            chunk[0], chunk[1], chunk[2], chunk[3]
        ]);

        for i in (0..32).rev() {
            bits.push(((value >> i) & 1) as u8);
        }
    }
    
    Some(bits)
}
