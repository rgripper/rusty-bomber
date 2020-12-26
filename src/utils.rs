use bevy::math::Vec3;

pub const TILE_WIDTH: f32 = 20.0;
pub const HALF_TILE_WIDTH: f32 = 10.0;

pub fn aabb_detection(x: f32, y: f32, one: Vec3) -> bool {
    one.x + TILE_WIDTH > x
        && x + TILE_WIDTH > one.x
        && one.y + TILE_WIDTH > y
        && y + TILE_WIDTH > one.y
}
