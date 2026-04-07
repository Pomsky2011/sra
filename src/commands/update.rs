use anyhow::Result;
use colored::Colorize;

pub fn run() -> Result<()> {
    println!("{}", "Checking for SRA package updates...".bold());
    // TODO: compare installed versions against SRA index, rebuild outdated packages
    println!("{}", "Not yet implemented".yellow());
    Ok(())
}
