#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub file: String,
    #[arg(long = "log", short, default_value = "warn")]
    pub log_level: log::LevelFilter,
}
