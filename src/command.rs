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
  name: Option<String>,
}

pub fn run_clyp_command(clyps_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
  let repo = ClypRepository::new(clyps_dir.to_string());
  let opts = Opts::from_args();

  if opts.clear {
    repo.clear_clyps();
  } else {
    save_or_read_clyp(opts.save, opts.read, opts.name, repo);
  }
  Ok(())
}

fn save_or_read_clyp(save: bool, read: bool, name: Option<String>, repo: ClypRepository) {
  let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

  if save {
    let content = ctx.get_contents().expect("could not read from clipboard");
    repo.save_clyp(get_name(name), content);
  } else if read {
    let content;
    if name.is_some() {
      content = repo.read_clyp(get_name(name));
    } else {
      content = repo.get_most_recent_clyp();
    }
    ctx
      .set_contents(content)
      .expect("could not add to clipboard");
  }
}
fn get_name(name: Option<String>) -> String {
  if name.is_some() {
    return name.unwrap();
  }
  return ".temporary".to_string();
}
