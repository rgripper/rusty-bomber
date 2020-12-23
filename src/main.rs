use bevy::prelude::*;
use bevy_rapier2d::rapier::{
    dynamics::RigidBodyBuilder,
    geometry::ColliderBuilder,
};
use bevy_rapier2d::render::RapierRenderPlugin;
use bevy_rapier2d::{
    na::{Vector, Vector2},
    physics::{RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::dynamics::RigidBodySet,
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(physical_setup.system())
        .add_resource(Map::first())
        .add_startup_stage(GMAE_SETUP, SystemStage::parallel()) // <--
        .add_startup_system_to_stage(GMAE_SETUP, game_setup_room.system())
        .add_stage(MOVEMENT, SystemStage::parallel())
        .add_system_to_stage(MOVEMENT, change_direction.system())
        .add_system_to_stage(MOVEMENT, player_movement.system())
        .run();
}
fn physical_setup(mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vector::y() * 0.0;
}
// TODO: We need a smart system to help us when player wants to move,
// the system will fix step size and related Animation.
const TILE_WIDTH: i32 = 20;
const HALF_TITLE_WIDTH: i32 = 10;
struct Wall;

struct Destructable;

struct Player {
    is_moving: bool,
}

struct Creature;

struct PermaWallMaterial(Handle<ColorMaterial>);

struct DestructableWallMaterial(Handle<ColorMaterial>);

struct FloorMaterial(Handle<ColorMaterial>);

struct PlayerMaterial(Handle<ColorMaterial>);

struct CreatureMaterial(Handle<ColorMaterial>);

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(PermaWallMaterial(
            materials.add(Color::rgb(0.2, 1.0, 0.7).into()),
        ))
        .insert_resource(DestructableWallMaterial(
            materials.add(Color::rgb(1.0, 1.0, 0.7).into()),
        ))
        .insert_resource(FloorMaterial(
            materials.add(Color::rgb(0.5, 1.0, 0.5).into()),
        ))
        .insert_resource(PlayerMaterial(
            materials.add(Color::rgb(0.7, 0.5, 1.0).into()),
        ))
        .insert_resource(CreatureMaterial(
            materials.add(Color::rgb(1.0, 0.3, 0.5).into()),
        ));
}

fn game_setup_room(
    commands: &mut Commands,
    perma_wall_material: Res<PermaWallMaterial>,
    map_resource: Res<Map>,
    destructable_wall_material: Res<DestructableWallMaterial>,
    player_material: Res<PlayerMaterial>,
    //mut wall_position: Query<(&Wall, &mut Transform)>,
) {
    let room_map = map_resource.map_value();

    for (row_index, row) in room_map.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            // Using match here makes it easier to extend the map
            match *cell {
                1 => {
                    commands
                        .spawn(SpriteBundle {
                            material: perma_wall_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(
                            RigidBodyBuilder::new_static()
                                .translation(
                                    col_index as f32 * TILE_WIDTH as f32,
                                    (room_map.len() - row_index - 1) as f32 * TILE_WIDTH as f32,
                                )
                                .lock_rotations()
                                .lock_translations(),
                        )
                        .with(
                            ColliderBuilder::cuboid(
                                HALF_TITLE_WIDTH as f32,
                                HALF_TITLE_WIDTH as f32,
                            )
                            .friction(0.0)
                            .restitution(0.0),
                        );
                }

                2 => {
                    commands
                        .spawn(SpriteBundle {
                            material: destructable_wall_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(
                            RigidBodyBuilder::new_static()
                                .translation(
                                    col_index as f32 * TILE_WIDTH as f32,
                                    (room_map.len() - row_index - 1) as f32 * TILE_WIDTH as f32,
                                )
                                .lock_rotations()
                                .lock_translations(),
                        )
                        .with(
                            ColliderBuilder::cuboid(
                                HALF_TITLE_WIDTH as f32,
                                HALF_TITLE_WIDTH as f32,
                            )
                            .friction(0.0)
                            .restitution(0.0),
                        );
                }
                // When setting each level, the player’s position should be set flexibly
                3 => {
                    commands
                        .spawn(SpriteBundle {
                            material: player_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            ..Default::default()
                        })
                        .with(Player { is_moving: false })
                        .with(Direction::Right)
                        .with(Velocity::new(5.0))
                        .with(
                            RigidBodyBuilder::new_dynamic()
                                .translation(
                                    col_index as f32 * TILE_WIDTH as f32,
                                    (room_map.len() - row_index - 1) as f32 * TILE_WIDTH as f32,
                                )
                                .lock_rotations(),
                        )
                        .with(
                            ColliderBuilder::cuboid(
                                HALF_TITLE_WIDTH as f32,
                                HALF_TITLE_WIDTH as f32,
                            )
                            .friction(0.0)
                            .restitution(0.0),
                        );
                }
                _ => continue,
            }
        }
    }
}

struct Map {
    value: Vec<Vec<i32>>,
}
impl Map {
    pub fn first() -> Self {
        let room_map = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 2, 2, 0, 0, 0, 0, 2, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 3, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 2, 0, 1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];
        Self { value: room_map }
    }

    pub fn map_value(&self) -> &Vec<Vec<i32>> {
        &self.value
    }
}
// ok
fn change_direction(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Direction, &mut Player)>,
) {
    for (mut direction, mut player) in query.iter_mut() {
        let movement_action = if keyboard_input.pressed(KeyCode::Left) {
            Some(Direction::Left)
        } else if keyboard_input.pressed(KeyCode::Down) {
            Some(Direction::Down)
        } else if keyboard_input.pressed(KeyCode::Up) {
            Some(Direction::Up)
        } else if keyboard_input.pressed(KeyCode::Right) {
            Some(Direction::Right)
        } else {
            None
        };
        if let Some(dir) = movement_action {
            *direction = dir;
            player.is_moving = true;
        } else {
            player.is_moving = false;
        }
    }
}

fn player_movement(
    time: Res<Time>,
    mut rigid_body_handle_query: Query<&mut RigidBodyHandleComponent>,
    mut rigid_body_set: ResMut<RigidBodySet>,
    mut query: Query<(Entity, &Velocity, &Player, &Direction), (Changed<Player>)>,
) {
    for (entity, velocity, player, direction) in query.iter_mut() {
        let rigid_body_handle = rigid_body_handle_query
            .get_component_mut::<RigidBodyHandleComponent>(entity)
            .unwrap();

        let rigid_body = rigid_body_set.get_mut(rigid_body_handle.handle()).unwrap();

        let mut x = 0.0;
        let mut y = 0.0;
        if player.is_moving {
            match direction {
                Direction::Left => x -= velocity.value * time.delta_seconds() * 500.0,
                Direction::Up => y += velocity.value * time.delta_seconds() * 500.0,
                Direction::Right => x += velocity.value * time.delta_seconds() * 500.0,
                Direction::Down => y -= velocity.value * time.delta_seconds() * 500.0,
            }
        }
        // println!("here! x:{},y:{}", x, y);
        rigid_body.set_linvel(Vector2::new(x, y), true);
    }
}

const GMAE_SETUP: &str = "game_setup";
const MOVEMENT: &str = "movement";

struct Velocity {
    value: f32,
}

impl Velocity {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
    pub fn set(&mut self, value: f32) {
        self.value = value;
    }
}
