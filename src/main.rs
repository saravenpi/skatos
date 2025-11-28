mod storage;
mod env_gen;
mod colors;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use env_gen::EnvGenerator;
use storage::Storage;
use colors::ColoredOutput;

#[derive(Parser)]
#[command(name = "skatos")]
#[command(about = "🛹 Generate environment files from skatos variables")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate .env file from skatos variables")]
    Env {
        #[arg(short, long, default_value = ".env")]
        output: String,
        #[arg(short, long, help = "Filter keys by prefix")]
        filter: Option<String>,
    },
    #[command(about = "Generate .env file from specific database")]
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
    #[command(about = "Export shell variables for eval (e.g., eval $(skatos export))")]
    Export {
        #[arg(short, long, help = "Database name (default: default)")]
        database: Option<String>,
        #[arg(short, long, help = "Filter keys by prefix")]
        filter: Option<String>,
    },
    #[command(about = "Set a key-value pair")]
    Set {
        #[arg(help = "Key name")]
        key: String,
        #[arg(help = "Value")]
        value: String,
    },
    #[command(about = "Get a value")]
    Get {
        #[arg(help = "Key name")]
        key: String,
    },
    #[command(about = "List all entries")]
    List,
    #[command(about = "List all keys")]
    Keys,
    #[command(about = "List all databases")]
    Dbs,
    #[command(about = "Delete a key")]
    Delete {
        #[arg(help = "Key name")]
        key: String,
    },
    #[command(about = "Backup all data to JSON file")]
    Backup {
        #[arg(short, long, default_value = "skatos_backup.json")]
        output: String,
    },
    #[command(about = "Restore data from JSON file")]
    Restore {
        #[arg(help = "Input JSON file path")]
        input: String,
    },
    #[command(about = "Import data from original skate (requires skate CLI)")]
    Import,
    #[command(about = "Generate shell completions")]
    Completions {
        #[arg(help = "Shell type (bash, zsh, fish, elvish, powershell)")]
        shell: Shell,
    },
}

/// Entry point for the skatos CLI application.
///
/// Parses command line arguments and executes the appropriate operation
/// based on the subcommand provided.
/// 
/// # Returns
/// 
/// Returns `Ok(())` on successful execution, or an error if any operation fails.
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let storage = Storage::new()?;

    match cli.command {
        Commands::Env { output, filter } => {
            EnvGenerator::generate_env_file(&storage, &output, filter.as_deref())?;
        }
        Commands::EnvFromDb { database, output } => {
            EnvGenerator::generate_from_db(&storage, &database, &output)?;
        }
        Commands::Preview { filter } => {
            EnvGenerator::show_preview(&storage, filter.as_deref())?;
        }
        Commands::Export { database, filter } => {
            EnvGenerator::export_shell(&storage, database.as_deref(), filter.as_deref())?;
        }
        Commands::Set { key, value } => {
            storage.set(&key, &value, None)?;
            println!("{} {}",
                ColoredOutput::success("Set"),
                ColoredOutput::format_key_value(&key, &value)
            );
        }
        Commands::Get { key } => {
            match storage.get(&key, None)? {
                Some(value) => println!("{}", ColoredOutput::value(&value)),
                None => println!("{} Key '{}' not found",
                    ColoredOutput::error("Error:"),
                    ColoredOutput::key(&key)
                ),
            }
        }
        Commands::List => {
            let entries = storage.list(None)?;
            if entries.is_empty() {
                println!("{}", ColoredOutput::warning("No entries found"));
            } else {
                for entry in entries {
                    println!("{}", ColoredOutput::format_key_value(&entry.key, &entry.value));
                }
            }
        }
        Commands::Keys => {
            let keys = storage.list_keys(None)?;
            if keys.is_empty() {
                println!("{}", ColoredOutput::warning("No keys found"));
            } else {
                for key in keys {
                    println!("{}", ColoredOutput::key(&key));
                }
            }
        }
        Commands::Dbs => {
            let dbs = storage.list_databases()?;
            if dbs.is_empty() {
                println!("{}", ColoredOutput::warning("No databases found"));
            } else {
                println!("{}", ColoredOutput::header("Available databases:"));
                for db in dbs {
                    println!("  {} {}",
                        "●".to_string(),
                        ColoredOutput::database(&db)
                    );
                }
            }
        }
        Commands::Delete { key } => {
            if storage.delete(&key, None)? {
                println!("{} Deleted {}",
                    ColoredOutput::success("Success:"),
                    ColoredOutput::key(&key)
                );
            } else {
                println!("{} Key '{}' not found",
                    ColoredOutput::error("Error:"),
                    ColoredOutput::key(&key)
                );
            }
        }
        Commands::Backup { output } => {
            EnvGenerator::backup_to_file(&storage, &output)?;
        }
        Commands::Restore { input } => {
            EnvGenerator::restore_from_file(&storage, &input)?;
        }
        Commands::Import => {
            println!("{}", ColoredOutput::info("Importing data from skate..."));
            match storage.import_from_skate() {
                Ok(count) => {
                    println!("{} Successfully imported {} entries",
                        ColoredOutput::success("Success:"),
                        ColoredOutput::count(count)
                    );
                }
                Err(e) => {
                    println!("{} Failed to import: {}",
                        ColoredOutput::error("Error:"),
                        e
                    );
                }
            }
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "skatos", &mut std::io::stdout());
            println!("\n{}", ColoredOutput::success("Completion script generated"));
            println!("{}", ColoredOutput::info("Add the output to your shell's configuration file"));
        }
    }

    Ok(())
}