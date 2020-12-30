use crate::components::*;
use bevy::prelude::Bundle;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    direction: Direction,
    velocity: Velocity,
    destructable: Destructable,
    bomb_power: BombPower,
    bomb_number: BombNumber,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player { is_moving: false },
            direction: Direction::Right,
            velocity: Velocity(1.0),
            destructable: Destructable::Player,
            bomb_power: BombPower(1),
            bomb_number: BombNumber { max: 1, current: 0 },
        }
    }
}
