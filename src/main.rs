use std::thread;

use anyhow::{Context, Result};
use clap::Parser;
use signal_hook::iterator::Signals;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let mut signals = Signals::new([signal_hook::consts::SIGINT, signal_hook::consts::SIGTERM])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}: ", sig)
        }
    });

    let args = Cli::parse();

    let file_content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    rust_cli::find_matches(&file_content, &args.pattern, std::io::stdout())?;
    Ok(())
}
