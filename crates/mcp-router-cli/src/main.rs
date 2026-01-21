use anyhow::Result;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Command to run")]
    command: String,

    #[arg(
        trailing_var_arg = true,
        allow_hyphen_values = true,
        help = "Arguments to pass to the command"
    )]
    args: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    tracing::info!("Running command `{} {:?}`", args.command, args.args);

    Ok(())
}
