mod clyprepo;
mod command;

use command::run_clyp_command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var_os("HOME").unwrap();
    let clyps_dir = std::path::PathBuf::from(home.as_os_str())
        .join(".clyp")
        .join("clyps");
    return run_clyp_command(clyps_dir);
}
