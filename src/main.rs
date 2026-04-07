mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "sra",
    about = "SRA — Source Repositories for Applications",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Search for packages in the SRA index
    Search {
        query: String,
    },
    /// Show information about a package
    Info {
        package: String,
    },
    /// Download, build, and install a package
    Install {
        packages: Vec<String>,
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },
    /// Remove an installed SRA package
    Remove {
        packages: Vec<String>,
    },
    /// Update all installed SRA packages
    Update,
    /// Submit a new ROLLPKG to the SRA index
    Submit {
        /// Path to the directory containing ROLLPKG.py
        path: std::path::PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Search { query }             => commands::search::run(&query),
        Command::Info { package }             => commands::info::run(&package),
        Command::Install { packages, yes }    => commands::install::run(&packages, yes),
        Command::Remove { packages }          => commands::remove::run(&packages),
        Command::Update                       => commands::update::run(),
        Command::Submit { path }              => commands::submit::run(&path),
    }
}
