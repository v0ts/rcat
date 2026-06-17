use anyhow::{Context, Ok, Result};
use clap::Parser;
use std::io::Write;

/// Search for a pattern in a file and display the lines that contain it. If you don't specify a pattern, it will display all lines.
#[derive(Parser)]
#[command(name = "rcat", bin_name = "rcat", version = "0.1.0")]
struct Cli {
    /// The path to the file to read
    path: std::path::PathBuf,

    /// The pattern to look for
    #[arg(short, long, required = false)]
    pattern: Option<String>,

    /// Ignore case when searching for the pattern
    #[arg(short, long, required = false)]
    ignore_case: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let pattern = args.pattern.unwrap_or(String::new());
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let stdout = std::io::stdout(); 
    let mut handle = stdout.lock();

    for line in content.lines() {
        if args.ignore_case {
            if line.to_lowercase().contains(&pattern.to_lowercase()) {
                writeln!(handle, "{}", line)?
            }
        } else {
            if line.contains(&pattern) {
                writeln!(handle, "{}", line)?
            }
        }
    }

    Ok(())
}
