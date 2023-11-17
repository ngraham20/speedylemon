use anyhow::{Result, Context};
use super::guild_wars_handler;
use super::course::Course;

pub fn run() -> Result<()> {
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
    println!("Name: {}, Racer Position: {:?}, Camera Position: {:?}", &data.racer.name, &data.racer.position, &data.camera.position);
    let course = Course::from_path(String::from("maps/TYRIACUP/TYRIA DIESSA PLATEAU.csv"))?;
    course.export_to_path(String::from("maps/RAEVENCUP/01-development.csv"))?;
    Ok(())
}