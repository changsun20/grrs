use anyhow::Result;
use clap::Parser;
use inquire::Text;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Try reading the file, fall back to interactive mode if failed
    let content = std::fs::read_to_string(&args.path).or_else(|e| {
        eprintln!("Failed to read {}: {}", args.path.display(), e);
        prompt_for_file_interactively()
    })?;

    // Search and print matching lines
    content
        .lines()
        .filter(|line| line.contains(&args.pattern))
        .for_each(|line| println!("{line}"));

    Ok(())
}

/// Prompts user to enter a file path interactively until successful read or cancellation
fn prompt_for_file_interactively() -> Result<String> {
    loop {
        let input_path = Text::new("Enter file path (ESC to cancel):")
            .prompt()
            .map_err(|_| anyhow::anyhow!("Operation cancelled by user"))?;

        match std::fs::read_to_string(&input_path) {
            Ok(content) => return Ok(content),
            Err(e) => eprintln!("Error reading file: {}: {}", input_path, e),
        }
    }
}
