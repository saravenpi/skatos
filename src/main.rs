mod skate;
mod env_gen;

use anyhow::Result;
use clap::{Parser, Subcommand};
use env_gen::EnvGenerator;
use skate::Skate;

#[derive(Parser)]
#[command(name = "skatos")]
#[command(about = "Generate environment files from skate variables")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate .env file from skate variables")]
    Env {
        #[arg(short, long, default_value = ".env")]
        output: String,
        #[arg(short, long, help = "Filter keys by prefix")]
        filter: Option<String>,
    },
    #[command(about = "Generate .env file from specific skate database")]
    EnvFromDb {
        #[arg(help = "Database name")]
        database: String,
        #[arg(short, long, default_value = ".env")]
        output: String,
    },
    #[command(about = "Preview environment variables without writing file")]
    Preview {
        #[arg(short, long, help = "Filter keys by prefix")]
        filter: Option<String>,
    },
    #[command(about = "Set a key-value pair in skate")]
    Set {
        #[arg(help = "Key name")]
        key: String,
        #[arg(help = "Value")]
        value: String,
    },
    #[command(about = "Get a value from skate")]
    Get {
        #[arg(help = "Key name")]
        key: String,
    },
    #[command(about = "List all skate entries")]
    List,
    #[command(about = "List all skate keys")]
    Keys,
    #[command(about = "List all skate databases")]
    Dbs,
    #[command(about = "Delete a key from skate")]
    Delete {
        #[arg(help = "Key name")]
        key: String,
    },
    #[command(about = "Backup all skate data to JSON file")]
    Backup {
        #[arg(short, long, default_value = "skate_backup.json")]
        output: String,
    },
    #[command(about = "Restore skate data from JSON file")]
    Restore {
        #[arg(help = "Input JSON file path")]
        input: String,
    },
}

/// Entry point for the skatos CLI application.
/// 
/// Parses command line arguments and executes the appropriate skate operation
/// based on the subcommand provided.
/// 
/// # Returns
/// 
/// Returns `Ok(())` on successful execution, or an error if any operation fails.
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Env { output, filter } => {
            EnvGenerator::generate_env_file(&output, filter.as_deref())?;
        }
        Commands::EnvFromDb { database, output } => {
            EnvGenerator::generate_from_db(&database, &output)?;
        }
        Commands::Preview { filter } => {
            EnvGenerator::show_preview(filter.as_deref())?;
        }
        Commands::Set { key, value } => {
            Skate::set(&key, &value)?;
            println!("Set {}={}", key, value);
        }
        Commands::Get { key } => {
            let value = Skate::get(&key)?;
            println!("{}", value);
        }
        Commands::List => {
            let entries = Skate::list()?;
            for entry in entries {
                println!("{}\t{}", entry.key, entry.value);
            }
        }
        Commands::Keys => {
            let keys = Skate::list_keys()?;
            for key in keys {
                println!("{}", key);
            }
        }
        Commands::Dbs => {
            let dbs = Skate::list_dbs()?;
            for db in dbs {
                println!("{}", db);
            }
        }
        Commands::Delete { key } => {
            Skate::delete(&key)?;
            println!("Deleted {}", key);
        }
        Commands::Backup { output } => {
            EnvGenerator::backup_to_file(&output)?;
        }
        Commands::Restore { input } => {
            EnvGenerator::restore_from_file(&input)?;
        }
    }

    Ok(())
}