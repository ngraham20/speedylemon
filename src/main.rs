mod guild_wars_handler;
mod livesplit_handler;
mod checkpoint;
mod speedylemon;

fn main() {
    if let Err(err) = speedylemon::run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}

// use mumblelink_reader::mumble_link_handler::MumbleLinkHandler;
// use mumblelink_reader::mumble_link::MumbleLinkReader;
// use std::{thread, time};

// fn main() {
//     let handler = MumbleLinkHandler::new().unwrap();
//     loop {
//         let linked_memory = handler.read().unwrap();
//         println!("{:?}", linked_memory);
//         thread::sleep(time::Duration::from_millis(50));
//     }
// }