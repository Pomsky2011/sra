use anyhow::{bail, Result};
use colored::Colorize;
use std::path::Path;

pub fn run(path: &Path) -> Result<()> {
    let rollpkg = path.join("ROLLPKG.py");
    if !rollpkg.exists() {
        bail!("no ROLLPKG.py found in {}", path.display());
    }
    println!("{} {}", "Submitting package from:".bold(), path.display().to_string().cyan());
    // TODO: validate ROLLPKG.py, push to SRA index git repo / API
    println!("{}", "Not yet implemented — SRA index URL not configured".yellow());
    Ok(())
}
