// rcli csv -i input.csv -o output.csv --header --d ',

use clap::Parser;
use rcli::{Opts, SubCommand, process_csv};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
