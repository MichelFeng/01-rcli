// rcli csv -i input.csv -o output.csv --header --d ',

use std::fs;

use clap::Parser;
use rcli::{
    Base64SubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand, process_csv,
    process_decode, process_encode, process_generate, process_genpass, process_sign,
    process_verify,
};
use zxcvbn::zxcvbn;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.no_uppercase,
                opts.no_lowercase,
                opts.no_number,
                opts.no_symbol,
            )?;
            println!("{}", password);

            // output the password strength
            let result = zxcvbn(&password, &[]);
            eprintln!("Password strength: {}", result.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format)?;
                println!("{}", encoded);
            }
            Base64SubCommand::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format)?;

                let decoded = String::from_utf8(decoded)?;
                println!("{}", decoded);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sig);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let keys = process_generate(opts.format)?;
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        fs::write(name, &keys[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = opts.output;
                        fs::write(name.join("ed25519.sk"), &keys[0])?;
                        fs::write(name.join("ed25519.pk"), &keys[1])?;
                    }
                }
            }
        },
    }
    Ok(())
}
