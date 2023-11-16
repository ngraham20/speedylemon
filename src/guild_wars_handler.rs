use mumblelink_reader::mumble_link_handler::MumbleLinkHandler;
use mumblelink_reader::mumble_link::{MumbleLinkReader, MumbleLinkDataReader};
use anyhow::{Context, Result};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct GuildwarsContext {
    pub server_address: [u8; 28],
    pub map_id: u32,
    pub map_type: u32,
    pub shard_id: u32,
    pub instance: u32,
    pub build_id: u32,
    pub ui_state: u32,
    pub compass_width: u16,
    pub compass_height: u16,
    pub compass_rotation: f32,
    pub player_x: f32,
    pub player_y: f32,
    pub map_center_x: f32,
    pub map_center_y: f32,
    pub map_scale: f32,
    pub process_id: u32,
    pub mount_index: u8,
}

// TODO: create a singleton shared-state reference struct with the specific required data
// Other structures that need this data can reference the single object

pub fn read_mumble() -> Result<()> {
    let handler = MumbleLinkHandler::new()?;
    let mumble_data = handler.read().context(format!("unable to read GW2 data from mumble API"))?;
    let racer_position = mumble_data.avatar;
    let gw2_data = mumble_data.read_context_into_struct::<GuildwarsContext>();
    let map_id = gw2_data.map_id;
    let name = mumble_data.identity.clone();
    println!("Racer Position: {:?}, Map: {}, Character: {}", racer_position, map_id, name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}