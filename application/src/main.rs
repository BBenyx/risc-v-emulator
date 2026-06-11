use std::env::args;

mod instruction;
mod file_reader;
mod cpu;

#[derive(Debug)]
enum MainError {
    NoArgumentError,
    NoFileFoundOnPathError,
}

fn main() -> Result<(), MainError> {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        return Err(MainError::NoArgumentError);
    }

    let cpu = cpu::Cpu {
        regs: [0; 64],
        pc: 0,
        memory: {
            match file_reader::read_file_bits(&args[1]) {
                Some(v) => v,
                None => return Err(MainError::NoFileFoundOnPathError),
            }
        },
    };


    //Just for testing
    println!("{:?}", cpu.memory);

    Ok(())
}
