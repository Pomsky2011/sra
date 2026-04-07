use anyhow::Result;
use colored::Colorize;

pub fn run(packages: &[String]) -> Result<()> {
    for pkg in packages {
        println!("{} {}", "Removing:".bold(), pkg.cyan());
    }
    // TODO: dpkg --remove
    println!("{}", "Not yet implemented".yellow());
    Ok(())
}
