use anyhow::Result;
use colored::Colorize;

pub fn run(packages: &[String], _yes: bool) -> Result<()> {
    for pkg in packages {
        println!("{} {}", "Installing:".bold(), pkg.cyan());
    }
    // TODO: fetch ROLLPKG.py, run rollpkg build, install .deb via dpkg
    println!("{}", "Not yet implemented".yellow());
    Ok(())
}
