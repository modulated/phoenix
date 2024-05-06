use clap::Parser;
use phoenix::{Args, VM};
use std::fs;

fn main() {
    let args = Args::parse();
    println!("Starting VM");
    let mut vm = VM::new();
    let pc_addr = u32::from_str_radix(&args.program_counter, 16).expect("Could not parse PC value");
    vm.set_pc(pc_addr);
    println!("Loading program");
    let rom = fs::read(&args.file)
        .unwrap_or_else(|_| panic!("Could not open provided file {}", args.file));
    vm.load(&rom);
    vm.run();
}
