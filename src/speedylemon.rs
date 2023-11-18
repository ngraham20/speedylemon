use anyhow::{Result, Context};
use super::guild_wars_handler;
use super::course::Course;
use log;
use device_query::{DeviceQuery, DeviceState, Keycode};

pub fn run() -> Result<()> {
    let device_state = DeviceState::new();

    // load checkpoint file from command argument or tui
    // get live mumble api data
    // as a base level functionality, output to the console when the next checkpoint is reached

    // main-loop
    // read the mumble data
    // update the racer position
    // check if the position is inside the next checkpoint

    let mut data = guild_wars_handler::GW2Data::new()?;
    data.init()?;
    data.update().context(format!("Failed to update GW2 Data"))?;
    log::debug!("Name: {}, Racer Position: {:?}, Camera Position: {:?}", &data.racer.name, &data.racer.position, &data.camera.position);
    let mut course = Course::from_path(String::from("maps/TYRIACUP/TYRIA DIESSA PLATEAU.csv"))?;
    course.export_to_path(String::from("maps/RAEVENCUP/01-development.csv"))?;

    let mut keys: Vec<Keycode> = Vec::new();
    while !keys.contains(&Keycode::Q) {
        data.update().context(format!("Failed to update GW2 Data"))?;
        let dst = super::util::euclidian_distance(data.racer.position, course.peek_next().point());
        log::debug!("Distance to checkpoint {}: {}", course.current_checkpoint, dst);

        keys = device_state.get_keys();
        if keys.contains(&Keycode::N) {
            course.collect_checkpoint();
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    log::info!("Terminating program.");
    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}