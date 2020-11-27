use bevy::prelude::*;
use std::cmp::*;

struct Position {
    x: i32,
    y: i32,
}

const TILE_WIDTH: u32 = 20;
const SIDE_TILE_COUNT: u32 = 10;

struct Wall;

struct Destructable;

struct Player {
    direction: Direction,
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

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(PermaWallMaterial(
        materials.add(Color::rgb(0.2, 1.0, 0.7).into()),
    ));
    commands.insert_resource(DestructableWallMaterial(
        materials.add(Color::rgb(1.0, 1.0, 0.7).into()),
    ));
    commands.insert_resource(FloorMaterial(
        materials.add(Color::rgb(0.5, 1.0, 0.5).into()),
    ));
    commands.insert_resource(PlayerMaterial(
        materials.add(Color::rgb(0.7, 0.5, 1.0).into()),
    ));
    commands.insert_resource(CreatureMaterial(
        materials.add(Color::rgb(1.0, 0.3, 0.5).into()),
    ));
}

fn game_setup_player(
    mut commands: Commands,
    player_material: Res<PlayerMaterial>,
    mut player_position: Query<(&Player, &mut Transform)>,
) {
    commands
        .spawn(SpriteComponents {
            material: player_material.0.clone(),
            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
            ..Default::default()
        })
        .with(Player {
            is_moving: false,
            direction: Direction::Right,
        })
        .with(Position { x: 0, y: 0 });

    // for (_player, mut transform) in &mut player_position.iter_mut() {
    //     transform.translation += Vec3::new(10.0, 0.0, 0.0);
    // }
}

fn game_setup_room(
    mut commands: Commands,
    perma_wall_material: Res<PermaWallMaterial>,
    destructable_wall_material: Res<DestructableWallMaterial>,
    mut wall_position: Query<(&Wall, &mut Transform)>,
) {
    let room_map = [
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        [1, 0, 2, 2, 0, 0, 0, 0, 2, 0, 1],
        [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ];

    for (row_index, row) in room_map.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            if *cell == 0 {
                continue;
            }
            commands
                .spawn(SpriteComponents {
                    material: if *cell == 2 {
                        destructable_wall_material.0.clone()
                    } else {
                        perma_wall_material.0.clone()
                    },
                    sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                    ..Default::default()
                })
                .with(Wall)
                .with(Position {
                    x: (TILE_WIDTH * (col_index as u32)) as i32,
                    y: (TILE_WIDTH * ((room_map.len() - row_index - 1) as u32)) as i32,
                });
        }
    }

    // for (_player, mut transform) in &mut player_position.iter_mut() {
    //     transform.translation += Vec3::new(10.0, 0.0, 0.0);
    // }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            pos.x as f32, //convert(pos.x as f32, window.width() as f32, ROOM_WIDTH as f32),
            pos.y as f32, //convert(pos.y as f32, window.height() as f32, ROOM_HEIGHT as f32),
            0.0,
        );
    }
}

fn change_position(
    position: &mut Mut<Position>,
    width: u32,
    height: u32,
    direction: &Direction,
    velocity: u32,
) {
    match direction {
        Direction::Left => {
            position.x = max(0, position.x - velocity as i32);
        }
        Direction::Right => {
            position.x = min(width as i32, position.x + velocity as i32);
        }
        Direction::Up => {
            position.y = min(height as i32, position.y + velocity as i32);
        }
        Direction::Down => {
            position.y = max(0, position.y - velocity as i32);
        }
    };
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    // mut game_over_events: ResMut<Events<GameOverEvent>>,
    mut players: Query<(Entity, &mut Player)>,
    mut player_query: Query<&mut Position>,
) {
    if let Some((player_entity, mut player)) = players.iter_mut().next() {
        let player_position = &mut player_query.get_mut(player_entity).unwrap();
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

        match movement_action {
            Some(direction) => {
                player.direction = direction;
                player.is_moving = true;
            }
            None => {
                player.is_moving = false;
            }
        }

        if player.is_moving {
            change_position(
                player_position,
                TILE_WIDTH * SIDE_TILE_COUNT,
                TILE_WIDTH * SIDE_TILE_COUNT,
                &player.direction,
                2,
            )
        }

        // if player_position.x < 0
        //     || player_position.y < 0
        //     || player_position.x as u32 >= ARENA_WIDTH
        //     || player_position.y as u32 >= ARENA_HEIGHT
        // {
        //     game_over_events.send(GameOverEvent);
        // }
    }
}

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup") // <--
        .add_startup_system_to_stage("game_setup", game_setup_player.system())
        .add_startup_system_to_stage("game_setup", game_setup_room.system())
        .add_system(position_translation.system())
        .add_system(player_movement.system())
        .add_plugins(DefaultPlugins)
        .run();
}
