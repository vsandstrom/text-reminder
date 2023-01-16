use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = 8)]
    pub hour: u32,
    #[arg(long, default_value_t = 30)]
    pub minute: u32,
    #[arg(long, default_value_t = 12)]
    pub interval: i64,
}
