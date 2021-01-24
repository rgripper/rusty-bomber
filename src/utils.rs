use bevy::math::{Vec2, Vec3};

use crate::constants::FLOOR_LAYER;

pub const SCALE: f32 = 3.0;
pub const TILE_WIDTH: f32 = 16.0 * SCALE;
pub const HALF_TILE_WIDTH: f32 = 8.0 * SCALE;
pub const FIX_DISTANCE: f32 = 5.0;

pub fn get_way_translation(player_position: Vec2) -> Option<(Vec3, Vec3)> {
    match player_position {
        Vec2 { x, y } if x % TILE_WIDTH == 0.0 && y % TILE_WIDTH == 0.0 => None,
        Vec2 { x, y } if x % TILE_WIDTH == 0.0 => {
            let one = Vec3::new(x, (y / TILE_WIDTH).floor() * TILE_WIDTH, FLOOR_LAYER);
            let two = Vec3::new(one.x, one.y + TILE_WIDTH, one.z);
            Some((one, two))
        }
        Vec2 { x, y } => {
            let one = Vec3::new((x / TILE_WIDTH).floor() * TILE_WIDTH, y, FLOOR_LAYER);
            let two = Vec3::new(one.x + TILE_WIDTH, one.y, one.z);
            Some((one, two))
        }
    }
}
pub fn vecs_xy_intersect(first: &Vec2, second: &Vec2) -> bool {
    first.x + TILE_WIDTH -FIX_DISTANCE> second.x
        && second.x + TILE_WIDTH-FIX_DISTANCE > first.x
        && first.y + TILE_WIDTH-FIX_DISTANCE > second.y
        && second.y + TILE_WIDTH-FIX_DISTANCE > first.y
}
#[inline(always)]
pub fn index_to_position(x: usize, y: usize, len: usize) -> Vec2 {
    Vec2::new(TILE_WIDTH * x as f32, TILE_WIDTH * (len - y - 1) as f32)
}
