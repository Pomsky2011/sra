use anyhow::Result;
use colored::Colorize;

pub fn run(query: &str) -> Result<()> {
    println!("{} {}", "Searching SRA for:".bold(), query.cyan());
    // TODO: fetch from SRA index API
    println!("{}", "Not yet implemented — index URL not configured".yellow());
    Ok(())
}
