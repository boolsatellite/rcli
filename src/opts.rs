use core::fmt;
use std::{str::FromStr};

use clap::{Args, Parser, Subcommand};


#[derive(Debug , Parser)]
#[command(name="rcli" , version , author , about , long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug , Subcommand)]
pub enum Subcommands {
    #[command(name="csv", about="show CSV or convert CSV to other formats")]
    Cvs(CsvOpts),
}

#[derive(Debug , Clone , Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

#[derive(Debug , Args)]
pub struct CsvOpts {
    #[arg(short , long , value_parser = verify_input_file)]
    pub input: String,
    #[arg(short , long)]
    pub output: Option<String>,
    #[arg(short , long , value_parser = parse_format)]
    pub format: OutputFormat,
    #[arg(short , long , default_value_t = ',')]
    pub delimiter: char,
    #[arg(long , default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(file_name: &str) ->Result<String , String> {
    if std::path::Path::new(file_name).exists() {
        Ok(file_name.into())
    } else {
        Err("File dose not exit".into())
    }
}

fn parse_format(format: &str) -> Result<OutputFormat , &'static str> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    } 
}

impl FromStr for OutputFormat {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> { 
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err("Invalid format"),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f , "{}", Into::<&'static str>::into(*self))
    }
}