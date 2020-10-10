mod clyprepo;
mod command;

use command::run_clyp_command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    return run_clyp_command("/Users/gbarker/GitHub/clyp/clyps");
}
