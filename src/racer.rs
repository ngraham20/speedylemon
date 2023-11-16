mod position {
    pub struct Position {
        x: f32,
        y: f32,
        z: f32,
    }
}

struct Racer {
    // beetle_angle is gathered from a mumblelink_reader::mumble_link::Position
    pub beetle_angle: f32,
    // camera_angle is gathered from a mumblelink_reader::mumble_link::Position
    pub camera_angle: f32,
    pub position: position::Position,
}