use clap::{Parser, Subcommand};
use std::env;
use tls::commands;
use tls::error::AppError;

#[derive(Parser)]
#[command(name = "tls")]
#[command(
    about = "Telescope CLI for scaffolding LLM evaluation projects",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a Telescope workspace in the current directory
    Init,
}

fn main() {
    let cli = Cli::parse();

    let result = handle_cli(cli);

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn handle_cli(cli: Cli) -> Result<(), AppError> {
    match cli.command {
        Commands::Init => {
            let cwd = env::current_dir()?;
            let report = commands::init(&cwd)?;

            println!("Initialized Telescope workspace at {}", cwd.display());
            if !report.created_paths.is_empty() {
                println!("Created:");
                for path in report.created_paths {
                    println!("- {}", path.display());
                }
            } else {
                println!("Workspace already contained the required layout; nothing new to create.");
            }

            if report.gitignore_updated {
                println!("Updated .gitignore with .telescope/ and .env");
            }

            Ok(())
        }
    }
}
