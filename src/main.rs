use clap::Parser;
use clap::Subcommand;

use crate::portfolio::Porfolio;
mod portfolio;

#[derive(Parser, Debug)]
#[command(name = "invest_folio")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Create { name: String },
    Open { name: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Create { name } => {
            Porfolio::new(None, name).await?;
            Ok(())
        }
        Commands::Open { name } => open(name),
    }
}
fn open(name: String) -> anyhow::Result<()> {
    println!("Searching for portfolio: {}", name);
    Ok(())
}
