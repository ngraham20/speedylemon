use anyhow::Result;
use super::guild_wars_handler;
use super::checkpoint;

pub fn run() -> Result<()> {
    // load checkpoint file from command argument or tui
    // get live mumble api data
    // as a base level functionality, output to the console when the next checkpoint is reached

    // guild_wars_handler::read_mumble()?;
    let checkpoints = checkpoint::load_checkpoint_file(String::from("maps/TYRIACUP/TYRIA DIESSA PLATEAU.csv"))?;
    checkpoint::write_checkpoints_to_file(checkpoints, String::from("CHECKPOINT.csv"));
    Ok(())
}