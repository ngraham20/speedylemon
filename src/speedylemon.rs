use anyhow::Result;
use super::guild_wars_handler;
use super::course::Course;

pub fn run() -> Result<()> {
    // load checkpoint file from command argument or tui
    // get live mumble api data
    // as a base level functionality, output to the console when the next checkpoint is reached

    // main-loop
    // read the mumble data
    // update the racer position
    // calculate the speed

    // dst = distance.euclidean(_pos, _lastPos)
    // total_distance = total_distance + dst
    // velocity = dst * 39.3700787 / timer

    // to calculate speed, the position and a timestamp should be saved together.
    // the speed is ds/dt, or the change in distance, divided by the change in time.

    guild_wars_handler::read_mumble()?;
    let course = Course::from_path(String::from("maps/TYRIACUP/TYRIA DIESSA PLATEAU.csv"))?;
    course.export_to_path(String::from("maps/RAEVENCUP/01-development.csv"))?;
    Ok(())
}