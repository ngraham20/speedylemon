use anyhow::Result;
use super::guild_wars_handler;
use super::course::Course;

pub fn run() -> Result<()> {
    // load checkpoint file from command argument or tui
    // get live mumble api data
    // as a base level functionality, output to the console when the next checkpoint is reached

    guild_wars_handler::read_mumble()?;
    let course = Course::from_path(String::from("maps/TYRIACUP/TYRIA DIESSA PLATEAU.csv"))?;
    course.export_to_path(String::from("maps/RAEVENCUP/01-development.csv"))?;
    Ok(())
}