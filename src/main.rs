use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opt) => {
            process_csv(&opt.input, &opt.output)?;
        }
    }
    Ok(())
}
