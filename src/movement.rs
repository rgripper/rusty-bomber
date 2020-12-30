use crate::{
    components::{Direction, Player, Velocity, Wall},
    utils::{aabb_detection, get_way_translation},
};
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
            let mut y = player_position.y;
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
                                y = player_position.y;
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
                                y = player_position.y;
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
                player_transform.translation.y = y;
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
            if way_translation.y == translation.y {
                return None;
            }

            // fix up or down distance
            // fix -> y value
            let way_y = way_translation.y;
            let y = translation.y;
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
