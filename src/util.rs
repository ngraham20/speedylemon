use crate::guild_wars_handler::Position;

pub fn euclidian_distance(a: Position, b: Position) -> f32 {
    ((a[0]-b[0]).powi(2) + (a[1]-b[1]).powi(2) + (a[2]-b[2]).powi(2)).sqrt()
}