mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use std::path::Path;

use self::{csv::CsvOpts, genpass::GenPassOpts};
use clap::Parser;

pub use self::{
    base64::{Base64Format, Base64Subcommand},
    csv::OutputFormat,
    http::{HttpSignOpts, HttpSubcommand},
    text::{TextSingFormat, TextSubcommand},
};

#[derive(Parser, Debug)]
#[clap(name = "rcli", version, author,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "csv subcommand")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "base64 subcommand")]
    Base64(Base64Subcommand),
    // #[command(subcommand)]
    // Text(TextSubcommand),
    #[command(subcommand)]
    Http(HttpSubcommand),
}

fn verify_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".to_string()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".to_string()));
        assert_eq!(
            verify_file("test.txt"),
            Err("File does not exist".to_string())
        );
    }
}
