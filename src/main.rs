// clap derive feature
// rcli csv -i input.csv -o output.json --header -d ','

use rcli::{Opts, SubCommand, process_csv};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,    
    };
    Ok(())
}