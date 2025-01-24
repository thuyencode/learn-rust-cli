use anyhow::{Context, Result};
use clap::Parser;
use indicatif::ProgressBar;
use std::io::{self, Write};
use std::{fs, thread, time};

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file '{}'", &args.path.display()))?;

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(time::Duration::from_millis(100));

    thread::sleep(time::Duration::from_secs(1));

    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line)?;
        }
    }

    spinner.finish();

    Ok(())
}
