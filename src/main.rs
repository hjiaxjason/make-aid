use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "make-aid"]
#[command(about = "The first-aid kit for complex Makefiles", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan directories and generate a self-documenting Makefile
    Init {
        #[arg(short, long, default_value = "C++")]
        language: String,
    },

    /// Audit an existing Makefile for circular dependencies, POSIX issues, and missing .PHONY tags
    Lint,

    /// Heal 'Ghost Header" errors by pruning orphaned compiler dependency files (.d)
    Heal,

    /// Monitor compilation flags and trigger clean rebuilds when they change
    Watch,
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Init { language } => {
            println!("{}", format!("Scanning project to generate a {} Makefile...", language).green());
        }
        Commands::Lint => {
            println!("{}", "Auditing Makefile for common bugs...".cyan());
        }
        Commands::Heal => {
            println!("{}", "Searching for orphaned header references...".yellow());
        }
        Commands::Watch => {
            println!("{}", "Monitoring compiler flags for changes...".blue());
        }
    }
}
