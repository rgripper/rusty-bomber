use crate::components::*;
use bevy::{core::Timer, prelude::Bundle};

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    direction: Direction,
    velocity: Velocity,
    bomb_power: BombPower,
    bomb_number: BombNumber,
    animation: Animation,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            direction: Direction::Right,
            velocity: Velocity {
                current: 0.0,
                max: 1.0,
            },
            bomb_power: BombPower(1),
            bomb_number: BombNumber { max: 1, current: 0 },
            animation: Animation(Timer::from_seconds(1.0, true)),
        }
    }
}
