mod track_selector;
mod speedylemon;
mod speedometer;
mod beetlerank;

use pretty_env_logger;
use log;
use std::cell::Cell;

thread_local!(static DEBUG: Cell<bool> = Cell::new(false));

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    if let Err(err) = speedylemon::run_test_program() {
        log::error!("Error: {:?}", err);
        std::process::exit(1);
    }
}