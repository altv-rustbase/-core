mod utils;
use utils::terminal::*;
use utils::config::*;

// CODE

#[altv::main]
fn main() -> altv::IntoVoidResult {
    let _config = ServerConfig::_load();
    Terminal::set_debug_status(_config.is_debug, _config.is_debug_detailed);

    // init server modules
    Terminal::debug("Server loading modules...");
}