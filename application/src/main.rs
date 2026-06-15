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

    let program = match file_reader::read_file_bits(&args[1]) {
                Some(v) => v,
                None => panic!("{:?}", MainError::NoFileFoundOnPathError),
            };
    let mut cpu = cpu::Cpu::new(program);

    cpu.run();

    Ok(())
}
