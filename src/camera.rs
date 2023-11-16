
use super::guild_wars_handler::Position;
pub struct Camera {
    pub position: Position,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: [0f32; 3],
        }
    }
}