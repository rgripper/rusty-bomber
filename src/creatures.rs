use crate::{
    components::{Direction, Player, PlayerPosition, Stop, Velocity, Wall},
    constants::PLAYER_LAYER,
    events::*,
    player::move_or_turn,
    ui::DrawBlinkTimer,
    utils::vecs_xy_intersect,
};
use bevy::prelude::*;
use bevy::{
    ecs::{Query, ResMut, SystemStage, With},
    sprite::ColorMaterial,
};
use rand::{seq::SliceRandom, thread_rng};

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

fn creature_movement(
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

pub trait CreatureSystems {
    fn creature_systems(&mut self) -> &mut Self;
}
impl CreatureSystems for SystemStage {
    fn creature_systems(&mut self) -> &mut Self {
        self.add_system(creature_player_collision.system())
            .add_system(creature_movement.system())
            .add_system(despawn_player.system())
    }
}

fn creature_player_collision(
    commands: &mut Commands,
    mut player_query: Query<(Entity, &mut PlayerPosition), With<Player>>,
    mut creature_query: Query<&mut Transform, With<Creature>>,
    mut game_over_events: ResMut<Events<GameOverEvent>>,
) {
    for (entity, player) in player_query.iter_mut() {
        let player_pos = &player.truncate();
        for creature_transform in creature_query.iter_mut() {
            if vecs_xy_intersect(&creature_transform.translation.truncate(), player_pos) {
                commands.insert(entity, StopAndFlashing::default());
                game_over_events.send(GameOverEvent(GameOverType::Defeat));
                // TODO: stop the game (stop movement system?)
            }
        }
    }
}

#[derive(Bundle)]
struct StopAndFlashing(Stop, DrawBlinkTimer, Timer);
impl Default for StopAndFlashing {
    fn default() -> Self {
        Self(
            Stop,
            DrawBlinkTimer(Timer::from_seconds(0.2, true)), //TODO:Slow here
            Timer::from_seconds(3.0, false),
        )
    }
}
fn despawn_player(
    commands: &mut Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Timer), (With<Stop>, With<DrawBlinkTimer>)>,
) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.tick(time.delta_seconds()).just_finished() {
            commands.despawn_recursive(entity);
        }
    }
}
