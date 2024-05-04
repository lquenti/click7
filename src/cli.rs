use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The path to the database (should end with .redb)
    #[arg(long)]
    pub database: PathBuf,

    /// The amount of digits printed
    /// (i.e. 7 digits => 0-9999999)
    #[arg(short, long, default_value_t = 7)]
    pub digits: u8,

    /// The padding around each of the digits in pixels
    #[arg(short, long, default_value_t = 20)]
    pub padding: u32,

    /// The thickness of the grey border
    #[arg(short, long, default_value_t = 20)]
    pub border: u32,

    /// port
    #[arg(long, default_value_t = 3000)]
    pub port: u32,
}
