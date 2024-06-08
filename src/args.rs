#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub file: String,
    #[arg(long = "log", short, default_value = "warn")]
    pub log_level: log::LevelFilter,
    #[arg(long = "pc", short, default_value = "00")]
    pub program_counter: String,
    #[arg(long = "usp", short)]
    pub user_stack_pointer: Option<String>,
    #[arg(long = "ssp", short)]
    pub system_stack_pointer: Option<String>,
}
