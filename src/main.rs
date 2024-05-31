mod basictui;
mod track_selector;
mod speedylemon;

use pretty_env_logger;
use log;
use std::cell::Cell;

thread_local!(static DEBUG: Cell<bool> = Cell::new(false));
thread_local!(static TRACK_SELECT: Cell<bool> = Cell::new(true));

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    if let Err(err) = speedylemon::run_program() {
        log::error!("Error: {:?}", err);
        std::process::exit(1);
    }
}