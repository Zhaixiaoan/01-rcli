use core::fmt;
use std::str::FromStr;

use clap::Parser;

use super::verify_file;

#[derive(Parser, Debug)]
pub enum TextSubcommand {
    #[command(about = "Sign a message using the Ed25519 algorithm")]
    Sign(TextSignOpts),

    #[command(about = "verify a message signature using the Ed25519 algorithm")]
    Verify(TextVerifyOpts),
}

#[derive(Parser, Debug)]
pub struct TextSignOpts {
    #[arg(short, long ,value_parser = verify_file,default_value = "-")]
    pub input: String,

    #[arg(short,long,value_parser = verify_file)]
    pub key: String,

    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSingFormat,
}

#[derive(Parser, Debug)]
pub struct TextVerifyOpts {
    #[arg(short, long ,value_parser = verify_file,default_value = "-")]
    pub input: String,

    #[arg(short,long,value_parser = verify_file)]
    pub key: String,

    #[arg(long, short)]
    pub sig: String,

    #[arg( long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSingFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSingFormat {
    Blake3,
    Ed25519,
}

pub fn parse_format(format: &str) -> Result<TextSingFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSingFormat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSingFormat::Blake3),
            "ed25519" => Ok(TextSingFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid base64 format")),
        }
    }
}

impl From<TextSingFormat> for &str {
    fn from(format: TextSingFormat) -> Self {
        match format {
            TextSingFormat::Blake3 => "blake3",
            TextSingFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSingFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
