use app_core::add;
use clap::{arg, Parser};
use std::fs::File;
use std::str::FromStr;
use tracing::Level;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    /// First number to be added (between 0 and 10)
    num1: u64,
    /// Two number to be added (between 0 and 10)
    num2: u64,
    /// log_level trace | debug | info | warn | error
    #[arg(long, default_value = "error")]
    log_level: String,
    /// Output path of log file
    #[arg(long, default_value = "./app_log.log")]
    log_path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_writer(File::create(&args.log_path)?)
        .with_max_level(Level::from_str(&args.log_level).unwrap_or(Level::ERROR))
        .init();

    match add(args.num1, args.num2) {
        Ok(res) => {
            tracing::info!("{}", res);
            Ok(())
        }
        Err(err) => {
            tracing::error!("{}", err);
            anyhow::bail!("{}", err)
        }
    }
}
