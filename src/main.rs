mod cpu;
mod gpu;
mod instruction;
mod memory_bus;

use std::env;
use std::fs::File;

use cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    let rom_path = env::args().nth(1).expect("No rom specified");
    let mut file = File::open(rom_path).expect("Could not open rom file");
    cpu.load(&mut file);
    cpu.step();
    println!("Hello, world!");
}
