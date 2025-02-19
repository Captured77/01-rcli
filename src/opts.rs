use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name=" ", version, author, about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
   pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name="csv", about="Show CSV, Convert CSV to other formats")]
    Csv(CsvOpts),
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser=verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value="output.json")] // "output.json".into()
    pub output: String,
 
    #[arg(short, long, default_value_t=',')]
    pub delimiter: char,

    // #[arg(short, long, default_value="false")]
    // header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("does not exist")
    }
}