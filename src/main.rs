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
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                "output.json".into()
            };
            process_csv(&opts.input, output , opts.format)?;
        }
    }
    Ok(())
}
// 37