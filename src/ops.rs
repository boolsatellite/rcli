use clap::Parser;

#[derive(Debug , Parser)]
#[command(name = "rcli" , version , author , about , long_about)]
pub struct Ops {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug , Parser)]
pub enum Subcommand {
    #[command(name = "csv" , about = "Show CSV or convert CSV to other formate")]
    Csv(CsvOpts),
}

#[derive(Debug , Parser)]
pub struct CsvOpts{
    #[arg(short , long , value_parser = verify_input_file)]
    pub input: String,

    #[arg(long , default_value = "output.json")]
    pub output: String,

    #[arg(long , default_value_t = ',')]
    pub delimiter: char,

    #[arg(long , default_value_t = true)]
    pub header: bool,
}


pub fn verify_input_file(filename: &str) ->Result<String , anyhow::Error> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        // Err("File does not exit".into())
        Err(anyhow::anyhow!("File not exit"))
    }
}