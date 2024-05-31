use crate::guild_wars_handler::Position;

pub struct Racer {
    pub position: Position,
    pub name: String,
}

impl Racer {
    pub fn new() -> Racer {
        Racer {
            position: [0f32; 3],
            name: String::new(),
        }
    }
}