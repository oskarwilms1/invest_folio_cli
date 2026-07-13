use clap::Parser;
use clap::Subcommand;

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

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Create { name } => create(name),
        Commands::Open { name } => open(name),
    }
}
fn create(name: String) -> anyhow::Result<()> {
    println!("Creating portfolio: {}", name);
    Ok(())
}
fn open(name: String) -> anyhow::Result<()> {
    println!("Searching for portfolio: {}", name);
    Ok(())
}
