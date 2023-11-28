use tokio::time::{sleep, Duration};

mod modules;
use modules::utils::terminal::*;
use modules::utils::config::*;
use crate::modules::database::DataBase;

mod api;
use api::users::events::UsersEvents;
use crate::api::users::Users;

// CODE

#[altv::main]
fn main() -> altv::IntoVoidResult {
    let _config = ServerConfig::_load();
    Terminal::set_debug_status(_config.is_debug, _config.is_debug_detailed);

    // init server modules
    Terminal::debug("Server loading modules...");

    main_async(&_config);

    Terminal::done("Server modules is loaded!");
}

#[tokio::main]
async fn main_async(_config:&MainConfig) {
    sleep(Duration::from_millis(500)).await;

    let _ = DataBase::_load(
        &_config.database.host,
        &_config.database.username,
        &_config.database.password,
        &_config.database.name
    ).await;

    // systems init
    UsersEvents::_register();
    Users::_init();
}