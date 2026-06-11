use std::fs;
use std::env::args;

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

    let bits = match read_file_bits(&args[1]) {
        Some(v) => v,
        None => return Err(MainError::NoFileFoundOnPathError)
    };

    println!("{:?}", bits);

    Ok(())
}

fn read_file_bits(path: &str) -> Option<Vec<u8>> {
    
    let bytes = fs::read(path);

    match bytes {
        Ok(value) => {

            let mut bits: Vec<u8> = Vec::new();
            for elem in value {

                for i in (0..8).rev() {

                    // Always shifting the bits to the right and reading the last element
                    bits.push((elem >> i) & 1);
                }
            }
            Some(bits)
        },
        Err(_) => None,
    }
}