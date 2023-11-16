use mumblelink_reader::mumble_link_handler::MumbleLinkHandler;
use mumblelink_reader::mumble_link::{MumbleLinkReader, MumbleLinkDataReader, MumbleLinkData};
use anyhow::{Context, Result, anyhow};

use super::racer::Racer;
use super::camera::Camera;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct GuildwarsContext {
    server_address: [u8; 28],
    map_id: u32,
    map_type: u32,
    shard_id: u32,
    instance: u32,
    build_id: u32,
    ui_state: u32,
    compass_width: u16,
    compass_height: u16,
    compass_rotation: f32,
    player_x: f32,
    player_y: f32,
    map_center_x: f32,
    map_center_y: f32,
    map_scale: f32,
    process_id: u32,
    mount_index: u8,
}

pub type Position = [f32; 3];

pub struct GW2Data {
    handler: MumbleLinkHandler,
    pub racer: Racer,
    pub camera: Camera,
    pub map_id: u32,
}

impl GW2Data {
    pub fn new() -> Result<GW2Data> {
        Ok(GW2Data {
            handler: MumbleLinkHandler::new()?,
            racer: Racer::new(),
            camera: Camera::new(),
            map_id: 0u32,
        })
    }
    #[cfg(target_family="windows")]
    pub fn update(&mut self) -> Result<()> {
        let data = self.handler.read().context(format!("unable to read GW2 data from mumble API"))?;
        self.racer.position = data.avatar.position;
        self.camera.position = data.camera.position;

        let gw2_data = data.read_context_into_struct::<GuildwarsContext>();

        self.map_id = gw2_data.map_id;
        Ok(())
    }

    #[cfg(target_family="unix")]
    pub fn update(&mut self) -> Result<()> {
        Err(anyhow!("Function not implemented for Unix"))
    }
}

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