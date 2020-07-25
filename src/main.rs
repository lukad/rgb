mod cpu;
mod gpu;
mod instruction;
mod memory_bus;

use cpu::CPU;

fn main() {
    let mut cpu = CPU::new();
    cpu.step();
    println!("Hello, world!");
}
