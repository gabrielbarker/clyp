use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use structopt::StructOpt;

mod clyprepo;
use clyprepo::ClypRepository;

/// Save and read clyps.
#[derive(StructOpt)]
struct Cli {
    /// Activate save mode
    #[structopt(short, long)]
    save: bool,
    /// Activate read mode
    #[structopt(short, long)]
    read: bool,
    /// The name of the clyp
    #[structopt()]
    name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let repo = ClypRepository::new("/Users/gbarker/GitHub/clyp/clyps".to_string());
    let name = Cli::from_args().name;

    if Cli::from_args().save {
        let content = ctx.get_contents().expect("could not read from clipboard");
        repo.save_clyp(name, content);
    } else if Cli::from_args().read {
        let content = repo.read_clyp(name);
        ctx.set_contents(content)
            .expect("could not add to clipboard");
    }

    Ok(())
}

#[cfg(test)]
mod path_tests {
    #[test]
    fn check_answer_validity() {
        assert_eq!(2 + 2, 4);
    }
}
