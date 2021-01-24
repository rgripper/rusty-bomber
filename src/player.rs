use crate::{
    components::{
        AnimateIndexs, Animation, BombNumber, BombPower, Destructible, Direction, Player, Stop,
        Velocity,
    },
    entitys::{create_dyn_rigid_body, create_player_collider},
    errors::querr_error_handler,
};

use anyhow::Result;
use bevy::{ecs::QueryError, prelude::*};
use bevy_rapier2d::{
    na::Vector2,
    physics::{ColliderHandleComponent, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};

pub trait PlayerSystems {
    fn player_systems(&mut self) -> &mut Self;
}
impl PlayerSystems for SystemStage {
    fn player_systems(&mut self) -> &mut Self {
        self
            // movement
            .add_system(movement.system().chain(querr_error_handler.system()))
            .add_system(stop_player.system())
            .add_system(for_player_add_collision_detection.system())
            // animate
            .add_system(animate_player.system())
            .add_system(velocity_to_animation.system())
    }
}
#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    direction: Direction,
    velocity: Velocity,
    bomb_power: BombPower,
    bomb_number: BombNumber,
    animation: Animation,
    destructible: Destructible,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player { is_moving: false },
            direction: Direction::Right,
            velocity: Velocity(150.0),
            bomb_power: BombPower(1),
            bomb_number: BombNumber { max: 1, current: 0 },
            animation: Animation(Timer::from_seconds(1.0, true)),
            destructible: Destructible::Player,
        }
    }
}
fn for_player_add_collision_detection(
    commands: &mut Commands,
    query: Query<
        (Entity, &Transform),
        (
            With<Player>,
            Without<RigidBodyBuilder>,
            Without<ColliderBuilder>,
            Without<RigidBodyHandleComponent>,
            Without<ColliderHandleComponent>,
        ),
    >,
) {
    for (entity, transform) in query.iter() {
        let translation = transform.translation;
        commands.insert(
            entity,
            (
                create_dyn_rigid_body(translation.x, translation.y),
                create_player_collider(entity),
            ),
        );
    }
}

fn movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &Velocity, &mut Direction, &mut Player), Without<Stop>>,
    mut rigid_body_handle_query: Query<&mut RigidBodyHandleComponent>,
    mut rigid_body_set: ResMut<RigidBodySet>,
) -> Result<(), QueryError> {
    for (entity, velocity, mut direction, mut player) in query.iter_mut() {
        let movement_action = if keyboard_input.pressed(KeyCode::Left) {
            //info!("left");
            Some(Direction::Left)
        } else if keyboard_input.pressed(KeyCode::Down) {
            //info!("down");
            Some(Direction::Down)
        } else if keyboard_input.pressed(KeyCode::Up) {
            //info!("up");
            Some(Direction::Up)
        } else if keyboard_input.pressed(KeyCode::Right) {
            //info!("right");
            Some(Direction::Right)
        } else {
            //println!("none");
            None
        };

        let rigid_body_handle =
            rigid_body_handle_query.get_component_mut::<RigidBodyHandleComponent>(entity)?;
        let linvel = match movement_action {
            Some(dir) => {
                //info!("pre direction:{:?}",direction);
                *direction = dir;
                player.is_moving = true;
                match dir {
                    Direction::Left => Vector2::new(-velocity.0, 0.0),
                    Direction::Up => Vector2::new(0.0, velocity.0),
                    Direction::Right => Vector2::new(velocity.0, 0.0),
                    Direction::Down => Vector2::new(0.0, -velocity.0),
                }
            }
            None => {
                player.is_moving = false;
                Vector2::new(0.0, 0.0)
            }
        };
        if let Some(rigid_body) = rigid_body_set.get_mut(rigid_body_handle.handle()) {
            rigid_body.set_linvel(linvel, true);
        } else {
            error!("Get rigid body fail!");
        }
        // while let Ok(proximity_event) = events.proximity_events.pop() {
        //     info!("Received proximity event: {:?}", proximity_event);
        // }
    }
    Ok(())
}

// animate
fn animate_player(
    time: Res<Time>,
    animate_date: Res<AnimateIndexs<Player>>,
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite, &Player, &Direction)>,
) {
    for (mut animation, mut sprite, _, direction) in query
        .iter_mut()
        .filter(|(_, _, player, _)| player.is_moving)
    {
        let indexs = match direction {
            Direction::Left => &animate_date.left,
            Direction::Up => &animate_date.up,
            Direction::Right => &animate_date.right,
            Direction::Down => &animate_date.down,
        };
        let mut should_turn = true;
        'contatine: for &idx in indexs.iter() {
            if sprite.index == idx {
                should_turn = false;
                break 'contatine;
            }
        }
        if should_turn {
            sprite.index = indexs[0];
        }
        animation.0.tick(time.delta_seconds());
        if animation.0.just_finished() {
            let indexs = match direction {
                Direction::Left => &animate_date.left,
                Direction::Up => &animate_date.up,
                Direction::Right => &animate_date.right,
                Direction::Down => &animate_date.down,
            };
            if sprite.index == indexs[0] {
                sprite.index = indexs[1];
            } else if sprite.index == indexs[1] {
                sprite.index = indexs[2];
            } else {
                sprite.index = indexs[0];
            }
            //info!("index:{}", sprite.index);
        }
    }
}
fn velocity_to_animation(
    mut query: Query<(&Velocity, &mut Animation), (With<Player>, Changed<Velocity>)>,
) {
    for (velocity, mut animation) in query.iter_mut() {
        animation.0.set_duration(1.0 / velocity.0 * 4.0);
    }
}
fn stop_player(mut query: Query<&mut Player, With<Stop>>) {
    for mut player in query.iter_mut() {
        player.is_moving = false;
    }
}
