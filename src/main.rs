use bevy::prelude::*;

const TILE_WIDTH: f32 = 20.0;
const HALF_TILE_WIDTH: f32 = 10.0;

struct Wall;
struct Way;
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
                    // way
                    commands.spawn((
                        Way,
                        Transform::from_translation(Vec3::new(
                            TILE_WIDTH * col_index as f32,
                            TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                            0.0,
                        )),
                    ));
                    // player
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
                        .with(Velocity(1.5));
                }
                _ => {
                    commands.spawn((
                        Way,
                        Transform::from_translation(Vec3::new(
                            TILE_WIDTH * col_index as f32,
                            TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                            0.0,
                        )),
                    ));
                }
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
    wall_position: Query<&Transform, (With<Wall>, Without<Player>)>,
    mut request_repair_events: ResMut<Events<RequestRepairEvent>>,
    fixed_move_event: Res<Events<FixedMoveEvent>>,
    mut fixed_move_event_reader: Local<EventReader<FixedMoveEvent>>,
    mut query: Query<(&Velocity, &Player, &Direction, &mut Transform), (Changed<Player>)>,
) {
    for (velocity, player, direction, mut player_transform) in query.iter_mut() {
        if player.is_moving {
            let mut x = player_transform.translation.x;
            let mut y = player_transform.translation.y;

            match direction {
                Direction::Left => x -= velocity.0,
                Direction::Up => y += velocity.0,
                Direction::Right => x += velocity.0,
                Direction::Down => y -= velocity.0,
            }

            let mut intersects = true;
            'for_loop: for transform in wall_position.iter() {
                let one = transform.translation;

                if aabb_detection(x, y, one) {
                    request_repair_events
                        .send(RequestRepairEvent(player_transform.translation, *direction));
                    for position in fixed_move_event_reader.iter(&fixed_move_event) {
                        match position {
                            FixedMoveEvent::HaveWay(p) => {
                                if aabb_detection(p.x, p.y, one) {
                                    intersects = false;
                                } else {
                                    x = p.x;
                                    y = p.y;
                                }
                            }
                            _ => intersects = false,
                        }
                    }
                    break 'for_loop;
                }
            }
            if intersects {
                player_transform.translation = Vec3::new(x, y, 0.0);
            }
        }
    }
}
fn aabb_detection(x: f32, y: f32, one: Vec3) -> bool {
    one.x + TILE_WIDTH > x
        && x + TILE_WIDTH > one.x
        && one.y + TILE_WIDTH > y
        && y + TILE_WIDTH > one.y
}
fn fix_player_translation(
    direction: Direction,
    translation: Vec3,
    way_transform: Vec3,
    threshold: f32,
) -> Option<Vec3> {
    match direction {
        Direction::Left | Direction::Right => {
            // fix up or down distance
            // fix -> y value
            let way_y = way_transform.y;
            let y = translation.y;
            if way_y - y < threshold {
                Some(Vec3::new(translation.x, way_y, 0.0))
            } else {
                None
            }
        }
        Direction::Up | Direction::Down => {
            // fix left or right distance
            // fix -> x value
            let way_x = way_transform.x;
            let x = translation.x;
            if way_x - x < threshold {
                Some(Vec3::new(way_x, translation.y, 0.0))
            } else {
                None
            }
        }
    }
}
fn road_detection(
    threshold: Res<Threshold>,
    mut fixed_move_events: ResMut<Events<FixedMoveEvent>>,
    mut request_repair_event_reader: Local<EventReader<RequestRepairEvent>>,
    request_repair_events: Res<Events<RequestRepairEvent>>,
    way_position: Query<&Transform, (With<Way>, Without<Wall>)>,
) {
    for RequestRepairEvent(position, dir) in
        request_repair_event_reader.iter(&request_repair_events)
    {
        let x = position.x;
        let y = position.y;

        'for_loop: for transform in way_position.iter() {
            let one = transform.translation;

            let collision_x = one.x + TILE_WIDTH > x && x + TILE_WIDTH > one.x;
            let collision_y = one.y + TILE_WIDTH > y && y + TILE_WIDTH > one.y;
            if collision_x && collision_y {
                if let Some(fixed_position) =
                    fix_player_translation(*dir, *position, one, threshold.0)
                {
                    fixed_move_events.send(FixedMoveEvent::HaveWay(fixed_position));
                } else {
                    fixed_move_events.send(FixedMoveEvent::NoWay);
                }
                break 'for_loop;
            }
        }
    }
}

const GMAE_SETUP: &str = "game_setup";
const MOVEMENT: &str = "movement";
struct RequestRepairEvent(Vec3, Direction);
enum FixedMoveEvent {
    HaveWay(Vec3),
    NoWay,
}
struct Velocity(f32);
struct Threshold(f32);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_resource(Map::first())
        // TODO: Strange Bugs are triggered when the threshold is below 20
        .add_resource(Threshold(20.0))
        .add_event::<FixedMoveEvent>()
        .add_event::<RequestRepairEvent>()
        .add_startup_stage(GMAE_SETUP, SystemStage::parallel()) // <--
        .add_startup_system_to_stage(GMAE_SETUP, game_setup_room.system())
        .add_stage(MOVEMENT, SystemStage::parallel())
        .add_system_to_stage(MOVEMENT, change_direction.system())
        .add_system_to_stage(MOVEMENT, player_movement.system())
        .add_system_to_stage(MOVEMENT, road_detection.system())
        .run();
}
