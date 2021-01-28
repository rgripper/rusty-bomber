use crate::{
    components::{AnimateIndexs, Animation, Destructible, Direction, Stop, Velocity},
    errors::querr_error_handler,
    ui::DrawBlinkTimer,
};
use bevy::ecs::{Query, ResMut, SystemStage, With};
use bevy::{ecs::QueryError, prelude::*};
use bevy_rapier2d::{
    na::Vector2, physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet,
};
use rand::{thread_rng, Rng};

#[derive(Bundle)]
pub struct CreatureBundle {
    creature: Creature,
    direction: Direction,
    velocity: Velocity,
    destructible: Destructible,
    animation: Animation,
}

impl Default for CreatureBundle {
    fn default() -> Self {
        Self {
            creature: Creature,
            direction: Direction::Right,
            velocity: Velocity(200.0),
            destructible: Destructible::Creature,
            animation: Animation(Timer::from_seconds(0.3, true)),
        }
    }
}

pub struct Creature;

const TURN_PROBABILITY: i32 = 4;
pub trait CreatureSystems {
    fn creature_systems(&mut self) -> &mut Self;
}
impl CreatureSystems for SystemStage {
    fn creature_systems(&mut self) -> &mut Self {
        self.add_system(
            creature_movement
                .system()
                .chain(querr_error_handler.system()),
        )
        .add_system(despawn_player.system())
        .add_system(animate_creature.system())
    }
}

fn creature_movement(
    mut query: Query<(Entity, &Velocity, &mut Direction), (With<Creature>, Without<Stop>)>,
    mut rigid_body_handle_query: Query<&mut RigidBodyHandleComponent>,
    mut rigid_body_set: ResMut<RigidBodySet>,
) -> Result<(), QueryError> {
    for (entity, velocity, mut direction) in query.iter_mut() {
        let rigid_body_handle =
            rigid_body_handle_query.get_component_mut::<RigidBodyHandleComponent>(entity)?;
        let mut rng = thread_rng();
        let n = rng.gen_range(0..=100);
        if n < TURN_PROBABILITY {
            // only change ocassionally
            *direction = rand::random();
        }
        let linvel = match *direction {
            Direction::Left => Vector2::new(-velocity.0, 0.0),
            Direction::Up => Vector2::new(0.0, velocity.0),
            Direction::Right => Vector2::new(velocity.0, 0.0),
            Direction::Down => Vector2::new(0.0, -velocity.0),
        };
        if let Some(rigid_body) = rigid_body_set.get_mut(rigid_body_handle.handle()) {
            //info!("OK!");
            rigid_body.set_linvel(linvel, true);
        } else {
            error!("Get rigid body fail!");
        }
    }
    Ok(())
}

#[derive(Bundle)]
pub struct StopAndFlashing(Stop, DrawBlinkTimer, Timer);
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
// animate
fn animate_creature(
    time: Res<Time>,
    animate_date: Res<AnimateIndexs<Creature>>,
    mut query: Query<
        (&mut Animation, &mut TextureAtlasSprite, &Direction),
        (With<Creature>, Without<Stop>),
    >,
) {
    for (mut animation, mut sprite, direction) in query.iter_mut() {
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
