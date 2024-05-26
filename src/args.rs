#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub file: String,
    #[arg(long = "log", short, default_value = "warn")]
    pub log_level: log::LevelFilter,
    #[arg(long = "pc", short, default_value = "0400")]
    pub program_counter: String,
    #[arg(long = "sp", short, default_value = "FFFFFE")]
    pub stack_pointer: String,
}
