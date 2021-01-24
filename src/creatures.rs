use crate::{
    components::{AnimateIndexs, Animation, Destructible, Direction, Player, Stop, Velocity},
    entitys::{create_creature_collider, create_dyn_rigid_body},
    errors::querr_error_handler,
    events::*,
    ui::DrawBlinkTimer,
    utils::vecs_xy_intersect,
};
use bevy::{ecs::QueryError, prelude::*};
use bevy::{
    ecs::{Query, ResMut, SystemStage, With},
    sprite::ColorMaterial,
};
use bevy_rapier2d::{
    na::Vector2,
    physics::{ColliderHandleComponent, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};
use rand::{seq::SliceRandom, thread_rng};

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
            animation: Animation(Timer::from_seconds(1.0 / 50.0, true)),
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
const TURN_PROBABILITY: f32 = 0.02;
fn creature_movement(
    mut query: Query<(Entity, &Velocity, &mut Direction), With<Creature>>,
    mut rigid_body_handle_query: Query<&mut RigidBodyHandleComponent>,
    mut rigid_body_set: ResMut<RigidBodySet>,
) -> Result<(), QueryError> {
    for (entity, velocity, mut direction) in query.iter_mut() {
        let mut rng = thread_rng();
        let rigid_body_handle =
            rigid_body_handle_query.get_component_mut::<RigidBodyHandleComponent>(entity)?;
        if rand::random::<f32>() < TURN_PROBABILITY {
            // only change ocassionally
            *direction = *DIRECTIONS.choose(&mut rng).unwrap()
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
fn for_creature_add_collision_detection(
    commands: &mut Commands,
    query: Query<
        (Entity, &Transform),
        (
            With<Creature>,
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
                create_creature_collider(entity),
            ),
        );
    }
}

pub trait CreatureSystems {
    fn creature_systems(&mut self) -> &mut Self;
}
impl CreatureSystems for SystemStage {
    fn creature_systems(&mut self) -> &mut Self {
        self.add_system(creature_player_collision.system())
            .add_system(for_creature_add_collision_detection.system())
            .add_system(
                creature_movement
                    .system()
                    .chain(querr_error_handler.system()),
            )
            .add_system(despawn_player.system())
            .add_system(animate_creature.system())
    }
}

fn creature_player_collision(
    commands: &mut Commands,
    mut player_query: Query<
        (Entity, &mut Transform),
        (With<Player>, Without<Stop>, Without<StopAndFlashing>),
    >,
    mut creature_query: Query<&mut Transform, With<Creature>>,
    mut game_over_events: ResMut<Events<GameEvents>>,
) {
    for (entity, player_transform) in player_query.iter_mut() {
        let player_pos = &player_transform.translation.truncate();
        for creature_transform in creature_query.iter_mut() {
            if vecs_xy_intersect(&creature_transform.translation.truncate(), player_pos) {
                commands.insert(entity, StopAndFlashing::default());
                game_over_events.send(GameEvents::GameOver);
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
// animate
fn animate_creature(
    time: Res<Time>,
    animate_date: Res<AnimateIndexs<Creature>>,
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite, &Direction), With<Creature>>,
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
