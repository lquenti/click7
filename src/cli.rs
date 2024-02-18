use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The amount of digits printed
    /// (i.e. 7 digits => 0-9999999)
    #[arg(short, long, default_value_t = 7)]
    pub digits: u8,

    /// The number to render
    #[arg(short)]
    pub number: u64,
}

