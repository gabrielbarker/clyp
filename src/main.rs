mod clyprepo;
mod command;

use command::run_clyp_command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clyps_dir = get_clyps_dir()?;
    return run_clyp_command(clyps_dir);
}

fn get_clyps_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let home = std::env::var_os("HOME").unwrap();
    let clyps_dir = std::path::PathBuf::from(home.as_os_str())
        .join(".clyp")
        .join("clyps");
    std::fs::create_dir_all(&clyps_dir)?;
    std::fs::File::create(clyps_dir.join(".temporary"))?;
    return Ok(clyps_dir);
}
