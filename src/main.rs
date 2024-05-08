use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64Subcommand, Opts,
    SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opt) => {
            let output = if let Some(output) = opt.output {
                output.clone()
            } else {
                format!("output.{}", opt.format)
            };

            process_csv(&opt.input, output, opt.format)?;
        }
        SubCommand::GenPass(opt) => {
            process_genpass(
                opt.length,
                opt.uppercase,
                opt.lowercase,
                opt.number,
                opt.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64Subcommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
