use bevy::math::{Vec2, Vec3};

use crate::constants::FLOOR_LAYER;

pub const SCALE: f32 = 3.0;
pub const TILE_WIDTH: f32 = 16.0 * SCALE;
pub const HALF_TILE_WIDTH: f32 = 8.0 * SCALE;

pub fn aabb_detection(x: f32, y: f32, one: Vec3) -> bool {
    // collide(
    //     Vec3::new(x, y, 0.0),
    //     Vec2::splat(20.0),
    //     one,
    //     Vec2::splat(20.0),
    // )
    // .is_some()
    one.x + TILE_WIDTH > x
        && x + TILE_WIDTH > one.x
        && one.y + TILE_WIDTH > y
        && y + TILE_WIDTH > one.y
}

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
    first.x + TILE_WIDTH > second.x
        && second.x + TILE_WIDTH > first.x
        && first.y + TILE_WIDTH > second.y
        && second.y + TILE_WIDTH > first.y
}
#[inline(always)]
pub fn index_to_position(x: usize, y: usize, len: usize, layer: f32) -> Vec3 {
    Vec3::new(
        TILE_WIDTH * x as f32,
        TILE_WIDTH * (len - y - 1) as f32,
        layer,
    )
}
