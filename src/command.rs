use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use structopt::StructOpt;

use super::clyprepo::ClypRepository;

/// Save and read clyps.
#[derive(StructOpt)]
struct Opts {
  /// Activate save mode
  #[structopt(short, long)]
  save: bool,
  /// Activate read mode
  #[structopt(short, long)]
  read: bool,
  /// Clear all saved clyps
  #[structopt(short, long)]
  clear: bool,
  /// The name of the clyp
  #[structopt(required_unless_one(&["clear", "help"]))]
  name: Option<String>,
}

pub fn run_clyp_command(clyps_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
  let repo = ClypRepository::new(clyps_dir.to_string());
  let opts = Opts::from_args();

  if opts.clear {
    repo.clear_clyps();
  } else {
    save_or_read_clyp(opts.save, opts.read, opts.name.unwrap(), repo);
  }
  Ok(())
}

fn save_or_read_clyp(save: bool, read: bool, name: String, repo: ClypRepository) {
  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

  if save {
    let content = ctx.get_contents().expect("could not read from clipboard");
    repo.save_clyp(name, content);
  } else if read {
    let content = repo.read_clyp(name);
    ctx
      .set_contents(content)
      .expect("could not add to clipboard");
  }
}
