use std::{fs::File, io::Read};

use base64::{
    Engine as _,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};

use crate::cli::Base64Format;

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };

    println!("{}", encoded);
    Ok(())
}

fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf),
    }?;

    let decoded = String::from_utf8(decoded)?;

    println!("{}", decoded);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_base64_encode() {
        let input = "fixtures/b64encode.txt";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_base64_decode() {
        let input: &str = "fixtures/b64decode.txt";
        let format = Base64Format::UrlSafe;
        assert!(process_decode(input, format).is_ok());
    }
}
