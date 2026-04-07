use anyhow::Result;
use colored::Colorize;

pub fn run(package: &str) -> Result<()> {
    println!("{} {}", "Package:".bold(), package.cyan());
    // TODO: fetch from SRA index API
    println!("{}", "Not yet implemented — index URL not configured".yellow());
    Ok(())
}
