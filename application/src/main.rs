
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

fn main() {

}