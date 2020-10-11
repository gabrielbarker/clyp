mod clyprepo;
mod command;

use command::run_clyp_command;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clyps_dir = get_clyps_dir()?;
    return run_clyp_command(clyps_dir);
}

fn get_clyps_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = std::env::var_os("HOME").unwrap();
    let clyps_dir = PathBuf::from(home.as_os_str()).join(".clyp").join("clyps");
    let default_clyp = clyps_dir.join(".default");
    std::fs::create_dir_all(&clyps_dir)?;
    if !default_clyp.exists() {
        std::fs::write(default_clyp, " ")?;
    }
    return Ok(clyps_dir);
}
