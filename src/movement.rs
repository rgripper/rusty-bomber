use crate::{components::{Direction, Player, Velocity, Wall}, constants::FIXED_DISTANCE, utils::{aabb_detection, get_way_translation}};
use bevy::prelude::*;

pub const MOVEMENT: &str = "movement";


pub trait MovementSystems {
    fn movement_systems(&mut self) -> &mut Self;
}
impl MovementSystems for SystemStage {
    fn movement_systems(&mut self) -> &mut Self {
        self.add_system(player_movement.system())
            .add_system(change_direction.system())
    }
}

fn player_movement(
    wall_position: Query<&Transform, (With<Wall>, Without<Player>)>,
    mut query: Query<(&Velocity, &Player, &Direction, &mut Transform)>,
) {
    for (velocity, player, direction, mut player_transform) in query.iter_mut() {
        if player.is_moving {
            let player_position = player_transform.translation;
            let mut x = player_position.x;
            let mut y = player_position.y - FIXED_DISTANCE;
            let distance = velocity.0;

            match direction {
                Direction::Left => x -= distance,
                Direction::Up => y += distance,
                Direction::Right => x += distance,
                Direction::Down => y -= distance,
            }

            let mut intersects = true;
            let mut have_way = false;
            for transform in wall_position.iter() {
                let wall_position = transform.translation;

                if aabb_detection(x, y, wall_position) {
                    if let Some((one, two)) = get_way_translation(player_position.truncate()) {
                        //info!("way:{}, one: {}, two: {}", wall_position, one, two);
                        if let Some((value, is_x)) = fix_player_translation(
                            direction,
                            player_position,
                            wall_position,
                            one,
                            12.0,
                        ) {
                            have_way = true;
                            if is_x {
                                if x > value && x - distance >= value {
                                    x -= distance;
                                } else if x < value && x + distance <= value {
                                    x += distance;
                                }
                                y = player_position.y - FIXED_DISTANCE;
                            } else {
                                if y < value && y + distance <= value {
                                    y += distance;
                                } else if y > value && y - distance >= value {
                                    y -= distance;
                                }
                                x = player_position.x;
                            }
                        } else if let Some((value, is_x)) = fix_player_translation(
                            direction,
                            player_position,
                            wall_position,
                            two,
                            12.0,
                        ) {
                            have_way = true;
                            if is_x {
                                if x > value && x - distance >= value {
                                    x -= distance;
                                } else if x < value && x + distance <= value {
                                    x += distance;
                                }
                                y = player_position.y - FIXED_DISTANCE;
                            } else {
                                if y < value && y + distance <= value {
                                    y += distance;
                                } else if y > value && y - distance >= value {
                                    y -= distance;
                                }
                                x = player_position.x;
                            }
                        }
                    }

                    intersects = false;
                }
            }

            if intersects || have_way {
                player_transform.translation.x = x;
                player_transform.translation.y = y + FIXED_DISTANCE;
            }

        }
    }
}


#[inline]
pub fn fix_player_translation(
    direction: &Direction,

    translation: Vec3,
    wall_translation: Vec3,
    way_translation: Vec3,
    threshold: f32,
) -> Option<(f32, bool)> {
    match direction {
        Direction::Left | Direction::Right => {
            if wall_translation.y == way_translation.y {
                return None;
            }
            if way_translation.y == translation.y- FIXED_DISTANCE{
                return None;
            }

            // fix up or down distance
            // fix -> y value
            let way_y = way_translation.y;
            let y = translation.y - FIXED_DISTANCE;
            // println!("way_y:{}, y:{},sub:{}",way_y,y,way_y - y);

            if (way_y - y).abs() < threshold {
                Some((way_y, false))
            } else {
                None
            }
        }
        Direction::Up | Direction::Down => {
            if wall_translation.x == way_translation.x {
                //info!("None1");
                return None;
            }
            if way_translation.x == translation.x {
                //info!("None2");
                return None;
            }
            // fix left or right distance
            // fix -> x value
            let way_x = way_translation.x;
            let x = translation.x;
            if (way_x - x).abs() < threshold {
                //info!("here! x:{}", way_x);
                Some((way_x, true))
            } else {
                None
            }
        }
    }
}

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

fn move_or_turn(
    unit_pos: &Vec2,
    direction: &Direction,
    wall_pos_query: &Query<&Transform, With<Wall>>,
) -> Option<Vec2> {
    let velocity_vec = get_velocity_vec(direction, 2.0);

    let threshold = 5.0;
    let new_unit_pos = *unit_pos + velocity_vec;
    let maybe_wall = wall_pos_query.iter().find(|wall_tranform| {
        vecs_xy_intersect(new_unit_pos, wall_tranform.translation.truncate())
    });

    match maybe_wall {
        None => Some(new_unit_pos),
        Some(wall_transform) => {
            let maybe_adjacent_cell_pos = get_adjacent_cell_entrance(
                direction,
                unit_pos,
                &wall_transform.translation.truncate(),
                threshold,
            )
            .map(|adjacent_cell_entrance| {
                let has_adjacent_wall = wall_pos_query.iter().any(|wall_tranform| {
                    vecs_xy_intersect(adjacent_cell_entrance, wall_tranform.translation.truncate())
                });

                if has_adjacent_wall {
                    None
                } else {
                    Some(adjacent_cell_entrance)
                }
            })
            .flatten();

            maybe_adjacent_cell_pos
        }
    }
}

fn get_adjacent_cell_entrance(
    direction: &Direction,
    unit_pos: &Vec2,
    wall_pos: &Vec2,
    threshold: f32,
) -> Option<Vec2> {
    let maybe_entrance = match direction {
        Direction::Left | Direction::Right => {
            let upper = wall_pos.y + TILE_WIDTH;
            let lower = wall_pos.y - TILE_WIDTH;

            if (upper - unit_pos.y) < threshold {
                Some(Vec2::new(unit_pos.x, upper))
            } else if (unit_pos.y - lower) < threshold {
                Some(Vec2::new(unit_pos.x, lower))
            } else {
                None
            }
        }
        Direction::Up | Direction::Down => {
            let right = wall_pos.x + TILE_WIDTH;
            let left = wall_pos.x - TILE_WIDTH;
            if (right - unit_pos.x) < threshold {
                Some(Vec2::new(right, unit_pos.y))
            } else if (unit_pos.x - left) < threshold {
                Some(Vec2::new(left, unit_pos.y))
            } else {
                None
            }
        }
    };

    maybe_entrance.map(|entrance| {
        let turn_boost = 2.0;
        let turn_boost_vec = match direction {
            Direction::Left => Vec2::new(-turn_boost, 0.0),
            Direction::Right => Vec2::new(turn_boost, 0.0),
            Direction::Up => Vec2::new(0.0, turn_boost),
            Direction::Down => Vec2::new(0.0, -turn_boost),
        };

        entrance + turn_boost_vec
    })
}

fn get_velocity_vec(direction: &Direction, speed: f32) -> Vec2 {
    match direction {
        Direction::Left => Vec2::new(-speed, 0.0),
        Direction::Up => Vec2::new(0.0, speed),
        Direction::Right => Vec2::new(speed, 0.0),
        Direction::Down => Vec2::new(0.0, -speed),
    }
}
