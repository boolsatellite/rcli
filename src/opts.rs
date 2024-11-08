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

#[derive(Debug , Args)]
pub struct CsvOpts {
    #[arg(short , long , value_parser = verify_input_file)]
    pub input: String,
    #[arg(short , long , default_value_t = String::from("output.json"))]
    pub output: String,
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