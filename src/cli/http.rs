use clap::{arg, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub enum HttpSubcommand {
    #[command(about = "Serve the HTTP server")]
    Serve(HttpSignOpts),
}

#[derive(Parser, Debug)]
pub struct HttpSignOpts {
    #[arg(short, long, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
