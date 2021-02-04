use anyhow::{anyhow, Result};
use bevy::prelude::*;
use bevy_rapier2d::{
    na::Vector2,
    physics::{ColliderHandleComponent, EventQueue, RigidBodyHandleComponent},
    rapier::{
        dynamics::RigidBodyBuilder,
        geometry::{ColliderBuilder, ColliderSet, ContactEvent::Started, InteractionGroups},
    },
};
use rand::{thread_rng, Rng};

use crate::{
    components::{Direction, *},
    creatures::{Creature, StopAndFlashing},
    errors::error_handler,
    events::GameEvents,
    utils::HALF_TILE_WIDTH,
};

const CREATURE_GROUPS: u16 = 0b0010;
const PLAYER_GROUPS: u16 = 0b0001;
const WALL_GROUPS: u16 = 0b0100;
const WAY_GROUPS: u16 = 0b1000;
const NONE_GROUPS: u16 = 0b0000;

pub trait PhysicsSystems {
    fn physics_systems(&mut self) -> &mut Self;
}
impl PhysicsSystems for SystemStage {
    fn physics_systems(&mut self) -> &mut Self {
        self.add_system(for_player_add_collision_detection.system())
            .add_system(for_wall_add_collision_detection.system())
            .add_system(for_way_add_collision_detection.system())
            .add_system(for_creature_add_collision_detection.system())
            .add_system(handle_contact_events.system().chain(error_handler.system()))
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
fn for_wall_add_collision_detection(
    commands: &mut Commands,
    query: Query<
        (Entity, &Transform),
        (
            With<Wall>,
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
                create_static_rigid_body(translation.x, translation.y),
                create_collider(entity),
            ),
        );
    }
}
fn for_way_add_collision_detection(
    commands: &mut Commands,
    query: Query<
        (Entity, &Transform),
        (
            With<Way>,
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
                create_static_rigid_body(translation.x, translation.y),
                create_way_collider(entity),
            ),
        );
    }
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

#[inline(always)]
pub fn get_velocity_vec(direction: &Direction, velocity: f32) -> Vector2<f32> {
    match direction {
        Direction::Left => Vector2::new(-velocity, 0.0),
        Direction::Up => Vector2::new(0.0, velocity),
        Direction::Right => Vector2::new(velocity, 0.0),
        Direction::Down => Vector2::new(0.0, -velocity),
    }
}
fn handle_contact_events(
    commands: &mut Commands,
    events: Res<EventQueue>,
    collider_set: Res<ColliderSet>,
    //query_pipeline: Res<QueryPipeline>,
    mut game_over_events: ResMut<Events<GameEvents>>,
    //mut rigid_body_handle_query: Query<&mut RigidBodyHandleComponent>,
    //mut rigid_body_set: ResMut<RigidBodySet>,
    mut query_set: QuerySet<(
        Query<
            (Option<&Player>, Option<&Direction>),
            (
                Or<(With<Player>, With<Creature>, With<Wall>)>,
                Without<Stop>,
            ),
        >,
        Query<&mut Direction>,
    )>,
) -> Result<()> {
    while let Ok(contact_event) = events.contact_events.pop() {
        match contact_event {
            Started(idxl, idxr) => {
                let entity_left = Entity::from_bits(
                    collider_set
                        .get(idxl)
                        .ok_or(anyhow!("get error!"))?
                        .user_data as u64,
                );
                let entity_right = Entity::from_bits(
                    collider_set
                        .get(idxr)
                        .ok_or(anyhow!("get error!"))?
                        .user_data as u64,
                );
                match query_set.q0().get(entity_left) {
                    Ok((Some(_), Some(_))) => {
                        // player
                        match query_set.q0().get(entity_right) {
                            Ok((None, Some(_))) => {
                                // creature
                                commands.insert(entity_left, StopAndFlashing::default());
                                game_over_events.send(GameEvents::GameOver);
                            }
                            Ok(_) => {}
                            Err(err) => {
                                error!("{:?}", err);
                            }
                        }
                    }
                    Ok((None, Some(_))) => {
                        // creature
                        match query_set.q0().get(entity_right) {
                            Ok((None, None)) => {
                                //wall
                                //TODO:Make creatures smarter
                                if let Ok(mut direction) = query_set.q1_mut().get_mut(entity_left) {
                                    let mut rng = thread_rng();
                                    let n = rng.gen_range(0..=100);
                                    if n <= 50 {
                                        // only change ocassionally
                                        *direction = rand::random();
                                    }
                                }
                            }
                            Ok((Some(_), Some(_))) => {
                                commands.insert(entity_left, StopAndFlashing::default());
                                game_over_events.send(GameEvents::GameOver);
                            }
                            Ok(_) => {}
                            Err(err) => {
                                error!("{:?}", err);
                            }
                        }
                    }
                    Ok((None, None)) => {
                        // wall
                        match query_set.q0().get(entity_right) {
                            Ok((None, Some(_))) => {
                                //TODO:Make creatures smarter
                                if let Ok(mut direction) = query_set.q1_mut().get_mut(entity_right)
                                {
                                    let mut rng = thread_rng();
                                    let n = rng.gen_range(0..=100);
                                    if n <= 50 {
                                        // only change ocassionally
                                        *direction = rand::random();
                                    }
                                }
                            }
                            Ok(_) => {}
                            Err(err) => {
                                error!("{:?}", err);
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(err) => {
                        error!("error:{:?}", err);
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}
#[inline(always)]
pub fn create_static_rigid_body(translation_x: f32, translation_y: f32) -> RigidBodyBuilder {
    RigidBodyBuilder::new_static()
        .translation(translation_x, translation_y)
}
#[inline(always)]
pub fn create_dyn_rigid_body(translation_x: f32, translation_y: f32) -> RigidBodyBuilder {
    RigidBodyBuilder::new_dynamic()
        .translation(translation_x, translation_y)
        .lock_rotations()
}
#[inline(always)]
pub fn create_collider(entity: Entity) -> ColliderBuilder {
    ColliderBuilder::cuboid(HALF_TILE_WIDTH, HALF_TILE_WIDTH)
        .friction(0.0)
        .restitution(0.0)
        .user_data(entity.to_bits() as u128)
}

#[inline(always)]
pub fn create_creature_collider(entity: Entity) -> ColliderBuilder {
    //ColliderBuilder::cuboid(HALF_TILE_WIDTH, HALF_TILE_WIDTH)

    ColliderBuilder::ball(HALF_TILE_WIDTH)
        .friction(0.0)
        .restitution(0.0)
        .user_data(entity.to_bits() as u128)
        .solver_groups(InteractionGroups::new(CREATURE_GROUPS, WALL_GROUPS))
}
#[inline(always)]
pub fn create_way_collider(entity: Entity) -> ColliderBuilder {
    ColliderBuilder::cuboid(HALF_TILE_WIDTH, HALF_TILE_WIDTH)
        .user_data(entity.to_bits() as u128)
        .solver_groups(InteractionGroups::new(WAY_GROUPS, NONE_GROUPS))
        .collision_groups(InteractionGroups::new(WAY_GROUPS, NONE_GROUPS))
}
#[inline(always)]
pub fn create_fire_collider(entity: Entity) -> ColliderBuilder {
    ColliderBuilder::cuboid(HALF_TILE_WIDTH, HALF_TILE_WIDTH)
        .friction(0.0)
        .restitution(0.0)
        .user_data(entity.to_bits() as u128)
        .solver_groups(InteractionGroups::none())
}
#[inline(always)]
pub fn create_ball_collider(entity: Entity) -> ColliderBuilder {
    ColliderBuilder::ball(HALF_TILE_WIDTH)
        .friction(0.0)
        .restitution(0.0)
        .sensor(true)
        .user_data(entity.to_bits() as u128)
}
#[inline(always)]
pub fn create_player_collider(entity: Entity) -> ColliderBuilder {
    //ColliderBuilder::cuboid(HALF_TILE_WIDTH, HALF_TILE_WIDTH)
    ColliderBuilder::ball(HALF_TILE_WIDTH)
        .friction(0.0)
        .restitution(0.0)
        .user_data(entity.to_bits() as u128)
        //.sensor(true)
        .solver_groups(InteractionGroups::new(PLAYER_GROUPS, WALL_GROUPS))
}

#[inline(always)]
pub fn create_sensor_collider(entity: Entity) -> ColliderBuilder {
    ColliderBuilder::cuboid(HALF_TILE_WIDTH, HALF_TILE_WIDTH)
        .sensor(true)
        .user_data(entity.to_bits() as u128)
}
#[inline(always)]
pub fn create_ball_sensor_collider(entity: Entity) -> ColliderBuilder {
    ColliderBuilder::ball(HALF_TILE_WIDTH)
        .sensor(true)
        .user_data(entity.to_bits() as u128)
}
