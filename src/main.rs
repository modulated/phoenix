use clap::Parser;
use log::info;
use phoenix::{Args, VM};
use simplelog::ConfigBuilder;
use std::fs;

fn main() {
    let conf = ConfigBuilder::new()
        .set_time_level(log::LevelFilter::Off)
        .set_thread_level(log::LevelFilter::Off)
        .set_location_level(log::LevelFilter::Off)
        .set_target_level(log::LevelFilter::Off)
        .set_max_level(log::LevelFilter::Off)
        .build();
    let _ = simplelog::SimpleLogger::init(log::LevelFilter::Trace, conf);
    let args = Args::parse();
    info!("Starting VM");
    let mut vm = VM::new();
    let pc_addr = u32::from_str_radix(&args.program_counter, 16).expect("Could not parse PC value");
    vm.set_pc(pc_addr);
    info!("PC set to {pc_addr:#X}");
    if let Some(ssp) = args.system_stack_pointer {
        let ssp = u32::from_str_radix(&ssp, 16).expect("Could not parse SSP value");
        vm.cpu.write_sp(ssp);
        info!("SSP set to {ssp:#X}");
    }
    if let Some(usp) = args.user_stack_pointer {
        let usp = u32::from_str_radix(&usp, 16).expect("Could not parse USP value");
        vm.cpu.write_usp(usp);
        info!("USP set to {usp:#X}");
    }
    info!("Loading program");
    let rom = fs::read(&args.file)
        .unwrap_or_else(|_| panic!("Could not open provided file {}", args.file));
    vm.load(&rom);
    info!("{} bytes loaded to RAM", rom.len());
    vm.run();
}
