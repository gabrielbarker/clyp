use anyhow::{Context, Result};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = &Cli::from_args().path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    println!("file content: {}", content);
    Ok(())
}

#[cfg(test)]
mod path_tests {
    #[test]
    fn check_answer_validity() {
        assert_eq!(2 + 2, 4);
    }
}
