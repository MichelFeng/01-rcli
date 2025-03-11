mod base64;
mod csv;
mod genpass;

use std::path::Path;

use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;

pub use self::{base64::Base64Format, base64::Base64SubCommand, csv::OutputFormat};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    // if input is "-" or file exists, return the filename
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("File not found: {}", filename))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".to_string()));
        assert_eq!(verify_input_file("*"), Err("File not found: *".to_string()));
        assert_eq!(
            verify_input_file("Cargo.toml"),
            Ok("Cargo.toml".to_string())
        );
        assert_eq!(
            verify_input_file("notfound.txt"),
            Err("File not found: notfound.txt".to_string())
        );
    }
}
