use bevy::prelude::*;

// TODO: should be one represent one tile
#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

const TILE_WIDTH: i32 = 20;
// TODD:should be removed this, it prevents scalability
const SIDE_TILE_COUNT: i32 = 10;

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
                        .with(Position {
                            x: col_index as i32,
                            y: (room_map.len() - row_index - 1) as i32,
                        });
                }

                2 => {
                    commands
                        .spawn(SpriteBundle {
                            material: destructable_wall_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(Position {
                            x: col_index as i32,
                            y: (room_map.len() - row_index - 1) as i32,
                        });
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
                        .with(Position {
                            x: col_index as i32,
                            y: (room_map.len() - row_index - 1) as i32,
                        })
                        .with(Velocity::new(0.1));
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

fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            (TILE_WIDTH * pos.x) as f32,
            (TILE_WIDTH * pos.y) as f32,
            0.0,
        );
    }
}
struct Map {
    height: i32,
    width: i32,
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
        Self {
            height: 11,
            width: 11,
            value: room_map,
        }
    }

    pub fn map_value(&self) -> &Vec<Vec<i32>> {
        &self.value
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn intersects(&self, x: i32, y: i32) -> bool {
        // x: (col_index as u32) as i32,
        // y: ((room_map.len() - row_index - 1) as u32) as i32,
        let len = self.value.len();
        if let Some(row) = self.value.get(len - y as usize - 1) {
            if let Some(num) = row.get(x as usize) {
                if *num == 0 || *num == 3 {
                    return true;
                }
            }
        }
        false
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
    map_resource: Res<Map>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Player, &Direction, &mut Position), (Changed<Player>)>,
) {
    for (mut velocity, player, direction, mut position) in query.iter_mut() {
        if player.is_moving {
            let mut x = position.x;
            let mut y = position.y;

            match direction {
                Direction::Left => x -= 1,
                Direction::Up => y += 1,
                Direction::Right => x += 1,
                Direction::Down => y -= 1,
            }

            // println!("{:?}",map_resource.intersects(x, y));
            // println!("{:?}", velocity.finished(time.delta_seconds()));
            if map_resource.intersects(x, y) && velocity.finished(time.delta_seconds()) {
                println!("here!");
                position.set(x, y);
            }
        }
    }
}

const GMAE_SETUP: &str = "game_setup";
const MOVEMENT: &str = "movement";

struct Velocity {
    timer: Timer,
}

impl Velocity {
    pub fn new(value: f32) -> Self {
        let timer = Timer::from_seconds(value, true);
        Self { timer }
    }
    pub fn set(&mut self, value: f32) {
        self.timer.set_duration(value);
    }
    pub fn finished(&mut self,delta: f32) -> bool {
        // TODO: here will be replaced by a better way
        self.timer.tick(delta).just_finished()
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
        .add_system(position_translation.system())
        .run();
}
