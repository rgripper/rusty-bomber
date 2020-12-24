use bevy::prelude::*;

const TILE_WIDTH: f32 = 20.0;
const HALF_TILE_WIDTH: f32 = 10.0;

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
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                0.0,
                            )),
                            ..Default::default()
                        })
                        .with(Wall);
                }

                2 => {
                    commands
                        .spawn(SpriteBundle {
                            material: destructable_wall_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                0.0,
                            )),
                            ..Default::default()
                        })
                        .with(Wall);
                }
                // When setting each level, the player’s position should be set flexibly
                3 => {
                    commands
                        .spawn(SpriteBundle {
                            material: player_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                0.0,
                            )),
                            ..Default::default()
                        })
                        .with(Player { is_moving: false })
                        .with(Direction::Right)
                        .with(Velocity::new(2.0));
                }
                _ => continue,
            }
        }
    }

    // for (_player, mut transform) in &mut player_position.iter_mut() {
    //     transform.translation += Vec3::new(10.0, 0.0, 0.0);
    // }
}
//  be related to map generate,should use resource to stored this
// const ROOM_WIDTH:u32 = TILE_WIDTH * 11;
// const ROOM_HEIGHT:u32 = TILE_WIDTH * 11;

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
            vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            vec![1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 1],
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
            //println!("left");
            Some(Direction::Left)
        } else if keyboard_input.pressed(KeyCode::Down) {
            // println!("down");
            Some(Direction::Down)
        } else if keyboard_input.pressed(KeyCode::Up) {
            //println!("up");
            Some(Direction::Up)
        } else if keyboard_input.pressed(KeyCode::Right) {
            //println!("right");
            Some(Direction::Right)
        } else {
            //println!("none");
            None
        };
        if let Some(dir) = movement_action {
            *direction = dir;
            player.is_moving = true;
            //println!("moving!");
        } else {
            player.is_moving = false;
            //println!("stop!");
        }
    }
}

fn player_movement(
    wall_position: Query<(&Transform), (With<Wall>, Without<Player>)>,
    mut query: Query<(&Velocity, &Player, &Direction, &mut Transform), (Changed<Player>)>,
) {
    for (velocity, player, direction, mut player_transform) in query.iter_mut() {
        if player.is_moving {
            let mut x = player_transform.translation.x;
            let mut y = player_transform.translation.y;

            match direction {
                Direction::Left => x -= velocity.value,
                Direction::Up => y += velocity.value,
                Direction::Right => x += velocity.value,
                Direction::Down => y -= velocity.value,
            }

            let mut intersects = true;
            for transform in wall_position.iter() {
                let one = transform.translation;

                let collision_x = one.x + TILE_WIDTH > x && x + TILE_WIDTH > one.x;
                let collision_y = one.y + TILE_WIDTH > y && y + TILE_WIDTH > one.y;
                if collision_x && collision_y {
                    // TODO: need smart fix step length here
                    
                    intersects = false;
                }
            }
            if intersects {
                player_transform.translation = Vec3::new(x, y, 0.0);
            }
        }
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

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_resource(Map::first())
        .add_startup_stage(GMAE_SETUP, SystemStage::parallel()) // <--
        .add_startup_system_to_stage(GMAE_SETUP, game_setup_room.system())
        .add_stage(MOVEMENT, SystemStage::serial())
        .add_system_to_stage(MOVEMENT, change_direction.system())
        .add_system_to_stage(MOVEMENT, player_movement.system())
        .run();
}
