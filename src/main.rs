// use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
// use log::*;
use sixtyeightkay::{Args, VM};
use clap::Parser;
use std::fs;

fn main() {    
    let args = Args::parse();    
    // let cfg = ConfigBuilder::new().set_time_level(LevelFilter::Off).build();
    // let _ = TermLogger::init(args.log_level, cfg, TerminalMode::Mixed, ColorChoice::Auto);
    println!("Starting VM");
    let mut vm = VM::new();
    println!("Loading program");
    let rom = fs::read(&args.file).unwrap_or_else(|_| panic!("Could not open provided file {}", args.file));
    vm.load(&rom);
    vm.run();
}
