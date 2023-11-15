use anyhow::Result;
use super::guild_wars_handler;

pub fn run() -> Result<()> {
    // load checkpoint file from command argument or tui
    // get live mumble api data
    // as a base level functionality, output to the console when the next checkpoint is reached

    guild_wars_handler::read_mumble()?;
    Ok(())
}