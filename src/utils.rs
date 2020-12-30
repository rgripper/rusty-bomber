use bevy::math::{Vec2, Vec3};

use crate::constants::{FIXED_DISTANCE, FLOOR_LAYER};

pub const TILE_WIDTH: f32 = 20.0;
pub const HALF_TILE_WIDTH: f32 = 10.0;

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
        Vec2 { x, y } if x % TILE_WIDTH == 0.0 && (y-FIXED_DISTANCE)% TILE_WIDTH == 0.0 => None,
        Vec2 { x, y } if x % TILE_WIDTH == 0.0 => {
            let one = Vec3::new(x, ((y-FIXED_DISTANCE)/ TILE_WIDTH).floor() * TILE_WIDTH, FLOOR_LAYER);
            let two = Vec3::new(one.x, one.y + TILE_WIDTH, one.z);
            Some((one, two))
        }
        Vec2 { x, y } => {
            let one = Vec3::new((x / TILE_WIDTH).floor() * TILE_WIDTH, (y-FIXED_DISTANCE), FLOOR_LAYER);
            let two = Vec3::new(one.x + TILE_WIDTH, one.y, one.z);
            Some((one, two))
        }
    }
}
