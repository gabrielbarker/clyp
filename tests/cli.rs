use assert_cmd::prelude::*;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;
use std::process::Command;

#[test]
fn save_and_read() -> Result<(), Box<dyn Error>> {
  let clipboard_contents = "test clipboard data";
  let mut ctx: ClipboardContext = ClipboardProvider::new()?;
  ctx.set_contents(clipboard_contents.to_string())?;

  let clyp_name = "clyp_name";
  execute_save_command(&clyp_name)?;
  ctx.set_contents("secondary test clipboard data".to_string())?;
  execute_read_command(&clyp_name)?;

  let actual_clipboard_contents = ctx.get_contents()?;
  assert_eq!(actual_clipboard_contents, clipboard_contents.to_string());
  Ok(())
}

fn execute_save_command(clyp_name: &str) -> Result<(), Box<dyn Error>> {
  return execute_command_with_flag_and_name("-s", clyp_name);
}

fn execute_read_command(clyp_name: &str) -> Result<(), Box<dyn Error>> {
  return execute_command_with_flag_and_name("-r", clyp_name);
}

fn execute_command_with_flag_and_name(flag: &str, name: &str) -> Result<(), Box<dyn Error>> {
  let mut save_command = Command::cargo_bin("clyp")?;
  save_command.arg(flag).arg(name).assert().success();
  Ok(())
}
