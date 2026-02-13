// snippet_cli_clap_20260213T1706.rs
// Topic: Building a CLI with clap (command-line argument parsing)
// Demonstrates subcommands, options, and positional arguments.

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "myapp")]
#[command(about = "A simple CLI tool demo", long_about = None)]
struct Cli {
    /// Optional configuration file
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,

    /// Verbosity level (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Enable debug mode
    #[arg(long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new item
    Add {
        /// The name of the item
        name: String,

        /// Quantity to add
        #[arg(short, long, default_value_t = 1)]
        count: u32,
    },

    /// Remove an existing item
    Remove {
        /// ID of the item to remove
        id: u32,
    },

    /// List all items
    List {
        /// Filter by status
        #[arg(short, long)]
        status: Option<String>,

        /// Output format: json or text
        #[arg(short, long, default_value = "text")]
        format: String,
    },
}

fn main() {
    let cli = Cli::parse();

    // Show configuration if provided
    if let Some(cfg) = &cli.config {
        println!("Using config file: {}", cfg);
    }

    // Verbosity
    match cli.verbose {
        0 => {}
        1 => println!("Verbose mode enabled"),
        2 => println!("More verbose output"),
        _ => println!("Debug-level verbosity"),
    }

    if cli.debug {
        println!("Debug mode ON");
        // println!("Parsed args: {:?}", cli);
    }

    match &cli.command {
        Commands::Add { name, count } => {
            println!("Adding {} x{}", name, count);
        }
        Commands::Remove { id } => {
            println!("Removing item with ID {}", id);
        }
        Commands::List { status, format } => {
            println!("Listing items (status: {:?}, format: {})", status, format);
        }
    }
}

/*
Dependencies for Cargo.toml:

[dependencies]
clap = { version = "4.0", features = ["derive"] }

Examples of usage:

- Basic with config and debug:
  cargo run --bin snippet_cli_clap_20260213T1706.rs --config settings.toml --debug

- Subcommand add:
  cargo run --bin snippet_cli_clap_20260213T1706.rs add "My Item" --count 5 -v

- Subcommand list:
  cargo run --bin snippet_cli_clap_20260213T1706.rs list --status active --format json

- Subcommand remove:
  cargo run --bin snippet_cli_clap_20260213T1706.rs remove 42

*/
