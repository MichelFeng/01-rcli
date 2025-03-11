use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long = "no-uppercase", default_value_t = false)]
    pub no_uppercase: bool,

    #[arg(long = "no-lowercase", default_value_t = false)]
    pub no_lowercase: bool,

    #[arg(long = "no-number", default_value_t = false)]
    pub no_number: bool,

    #[arg(long = "no-symbol", default_value_t = false)]
    pub no_symbol: bool,
}
