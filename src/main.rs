use clap::{Parser};
use rcli::{process_csv, Opts};
use rcli::Subcommands;
use anyhow;

// rcli csv -i input.csv -o output.json 



fn main() ->anyhow::Result<()> {
    let ops = Opts::parse();
    println!("{:?}", ops);
    match ops.cmd {
        Subcommands::Cvs(opts) => {
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}