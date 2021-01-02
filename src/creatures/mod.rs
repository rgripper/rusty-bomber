use crate::{
    components::{Direction, Player, PlayerPosition, Velocity, Wall},
    constants::PLAYER_LAYER,
    movement::move_or_turn,
};
use bevy::{
    ecs::{Query, ResMut, With},
    prelude::{info, Assets, Handle, Transform},
    render::color::Color,
    sprite::ColorMaterial,
};
use rand::{seq::SliceRandom, thread_rng};

use crate::components::*;
use bevy::{core::Timer, prelude::Bundle};

#[derive(Bundle)]
pub struct CreatureBundle {
    creature: Creature,
    direction: Direction,
    velocity: Velocity,
}

impl Default for CreatureBundle {
    fn default() -> Self {
        Self {
            creature: Creature,
            direction: Direction::Right,
            velocity: Velocity {
                current: 0.0,
                max: 1.0,
            },
        }
    }
}

pub struct CreatureMaterial(pub Handle<ColorMaterial>);

pub struct Creature;

// could be done with a crate
const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

pub fn creature_movement(
    mut query: Query<(&Velocity, &mut Direction, &mut Transform), With<Creature>>,
    wall_pos_query: Query<&Transform, With<Wall>>,
) {
    let turn_probability = 0.02;
    for (_, mut direction, mut creature_position) in query.iter_mut() {
        let mut rng = thread_rng();

        match move_or_turn(
            &creature_position.translation.truncate(),
            &direction,
            &wall_pos_query,
        ) {
            Some(new_position) => {
                if rand::random::<f32>() < turn_probability {
                    // only change ocassionally
                    *direction = *DIRECTIONS.choose(&mut rng).unwrap()
                }
                creature_position.translation = new_position.extend(PLAYER_LAYER);
            }
            None => {
                // always change. Yes, need to filter out current position
                *direction = *DIRECTIONS.choose(&mut rng).unwrap();
            }
        }
    }
}
