use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// Activate copy mode
    #[structopt(short, long)]
    copy: bool,
    /// Activate paste mode
    #[structopt(short, long)]
    paste: bool,
    /// The name of the clyp
    #[structopt()]
    name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clyps_folder = "/Users/gbarker/GitHub/clyp/clyps";
    let path: std::path::PathBuf = [clyps_folder, &Cli::from_args().name].iter().collect();
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    if Cli::from_args().copy {
        let content = ctx.get_contents().expect("could not read from clipboard");
        save_to_file(path, content);
    } else if Cli::from_args().paste {
        let content = read_from_file(path);
        ctx.set_contents(content)
            .expect("could not add to clipboard");
    }
    Ok(())
}

fn save_to_file(path: std::path::PathBuf, content: String) {
    std::fs::write(path, content).expect("Unable to write file");
}

fn read_from_file(path: std::path::PathBuf) -> String {
    let content =
        std::fs::read_to_string(&path).expect(&format!("could not read file `{}`", path.display()));
    return content;
}

#[cfg(test)]
mod path_tests {
    #[test]
    fn check_answer_validity() {
        assert_eq!(2 + 2, 4);
    }
}
