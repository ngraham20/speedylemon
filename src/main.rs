mod guild_wars_handler;
mod livesplit_handler;
mod checkpoint;
mod course;
mod speedylemon;
mod racer;
mod camera;

fn main() {
    if let Err(err) = speedylemon::run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}