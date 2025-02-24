use clap::{Parser, Subcommand};
use candid::Principal;
use ic_agent::Agent;

#[derive(Parser)]
#[clap(version = "1.0")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send tokens to another address
    Send {
        to: String,
        amount: u64,
    },
    /// Check balance of an address
    Balance {
        address: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let agent = Agent::builder()
        .with_url("http://localhost:8000")
        .build()?;

    match cli.command {
        Commands::Send { to, amount } => {
            let to_principal = Principal::from_text(to)?;
            // Call canister's transfer method here
            println!("Sent {} tokens to {}", amount, to_principal);
        }
        Commands::Balance { address } => {
            let principal = Principal::from_text(address)?;
            // Call canister's get_balance method here
            println!("Balance for {}: [FETCHED]", principal);
        }
    }

    Ok(())
}