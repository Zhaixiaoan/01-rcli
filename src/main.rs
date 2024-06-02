use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    Base64Subcommand, HttpSubcommand, Opts, SubCommand,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
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
        // SubCommand::Text(subcmd) => match subcmd {
        //     TextSubcommand::Sign(opts) => match opts.format {
        //         TextSingFormat::Blake3 => {
        //             process_text_sign(&opts.input, &opts.key, opts.format)?;
        //         }
        //         TextSingFormat::Ed25519 => {
        //             println!("Ed25519");
        //         }
        //     },
        //     TextSubcommand::Verify(opts) => {
        //         print!("{:?}", opts);
        //     }
        // },
        SubCommand::Http(subcmd) => match subcmd {
            HttpSubcommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}
