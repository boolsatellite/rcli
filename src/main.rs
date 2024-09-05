use std::{error::Error, fs::ReadDir, io::{self, BufReader}, process, result};
use anyhow::Ok;
use clap::{command, Parser};
use cli::{process_csv, Ops, Subcommand};
use serde::{Serialize , Deserialize};
use std::fs;

// rcli csv -i input.csv -o output.json --header -d '.'





fn main() -> anyhow::Result<()> {
    let ops =  Ops::parse();
    match ops.cmd {
        Subcommand::Csv(ops) => {
            process_csv(&ops.input , &ops.output)?;
        }
    }
    Ok(())
}