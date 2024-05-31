mod guild_wars_handler;
mod checkpoint;
mod course;
mod speedylemon;
mod racer;
mod camera;
mod util;
// mod lemontui;
mod racelog;
mod splits;
mod beetlerank;
mod basictui;
mod track_selector;

use pretty_env_logger;
use log;

fn main() {
    pretty_env_logger::init();

    if let Err(err) = speedylemon::run_track_selector() {
        log::error!("Error: {:?}", err);
        std::process::exit(1);
    }
}