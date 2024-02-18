use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The amount of digits printed
    /// (i.e. 7 digits => 0-9999999)
    #[arg(short, long, default_value_t = 7)]
    pub digits: u8,

    /// The number to render (only for debug rn)
    #[arg(short)]
    pub number: u32,

    /// The padding around each of the digits in pixels
    #[arg(short, long, default_value_t = 20)]
    pub padding: u32,

    /// The thickness of the grey border
    #[arg(short, long, default_value_t = 20)]
    pub border: u32,
}
