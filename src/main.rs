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
    /// Execute evaluation blocks and write run logs
    Run {
        /// Include metrics directory in the run
        #[arg(long)]
        with_metrics: bool,
        /// Target a specific block id
        #[arg(long)]
        id: Option<String>,
    },
    /// Evaluate the latest run log
    Eval,
    /// Generate a Markdown report from the latest run/eval pair
    Report,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    let result = handle_cli(cli).await;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn handle_cli(cli: Cli) -> Result<(), AppError> {
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
        Commands::Run { with_metrics, id } => {
            let cwd = env::current_dir()?;
            let run_path = commands::run(&cwd, with_metrics, id.as_deref()).await?;
            println!("Wrote run log to {}", run_path.display());
            Ok(())
        }
        Commands::Eval => {
            let cwd = env::current_dir()?;
            let eval_path = commands::eval(&cwd).await?;
            println!("Wrote eval log to {}", eval_path.display());
            Ok(())
        }
        Commands::Report => {
            let cwd = env::current_dir()?;
            let report_path = commands::report(&cwd)?;
            println!("Wrote report to {}", report_path.display());
            Ok(())
        }
    }
}
