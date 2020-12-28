use crate::Direction;
use crate::*;

pub const MOVEMENT: &str = "movement";

pub fn player_movement(
    // time:Res<Time>,
    wall_position: Query<&Transform, (With<Wall>, Without<Player>)>,
    mut request_repair_events: ResMut<Events<RequestRepairEvent>>,
    fixed_move_event: Res<Events<FixedMoveEvent>>,
    mut fixed_move_event_reader: Local<EventReader<FixedMoveEvent>>,
    mut query: Query<(&Velocity, &Player, &Direction, &mut Transform), Changed<Player>>,
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
            let mut have_way = false;
            for transform in wall_position.iter() {
                let one = transform.translation;

                if aabb_detection(x, y, one) {
                    request_repair_events.send(RequestRepairEvent(
                        player_transform.translation,
                        *direction,
                        one,
                    ));
                    for position in fixed_move_event_reader.iter(&fixed_move_event) {
                        match position {
                            FixedMoveEvent::HaveWay(p, is_x) => {
                                have_way = true;
                                if *is_x {
                                    if x > p.x && x - velocity.0 >= p.x {
                                        x -= velocity.0;
                                    }

                                    if x < p.x && x + velocity.0 <= p.x {
                                        x += velocity.0;
                                    }
                                } else {
                                    if y < p.y && y + velocity.0 <= p.y {
                                        y += velocity.0;
                                    }
                                    if y > p.y && y - velocity.0 >= p.y {
                                        y -= velocity.0;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    intersects = false;
                }
            }

            // println!("x: {},y: {}", x, y);
            if intersects || have_way {
                player_transform.translation.x = x;
                player_transform.translation.y = y;
            }
        }
    }
}

pub fn fix_player_translation(
    direction: Direction,
    translation: Vec3,
    wall_translation: Vec3,
    way_translation: Vec3,
    threshold: f32,
) -> Option<(Vec3, bool)> {
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
                Some((Vec3::new(translation.x, way_y, 0.0), false))
            } else {
                None
            }
        }
        Direction::Up | Direction::Down => {
            if wall_translation.x == way_translation.x {
                return None;
            }
            if way_translation.x == translation.x {
                return None;
            }
            // fix left or right distance
            // fix -> x value
            let way_x = way_translation.x;
            let x = translation.x;
            if (way_x - x).abs() < threshold {
                Some((Vec3::new(way_x, translation.y, 0.0), true))
            } else {
                None
            }
        }
    }
}
pub fn road_detection(
    threshold: Res<Threshold>,
    mut fixed_move_events: ResMut<Events<FixedMoveEvent>>,
    mut request_repair_event_reader: Local<EventReader<RequestRepairEvent>>,
    request_repair_events: Res<Events<RequestRepairEvent>>,
    mut have_player_way_position_reader: Local<EventReader<HavePlayerWayEvent>>,
    have_player_way_position: Res<Events<HavePlayerWayEvent>>,
) {
    for RequestRepairEvent(position, dir, wall_position) in
        request_repair_event_reader.iter(&request_repair_events)
    {
        for transform in have_player_way_position_reader.iter(&have_player_way_position) {
            let one = transform.0;
            if let Some((fixed_position, is_x)) =
                fix_player_translation(*dir, *position, *wall_position, one, threshold.0)
            {
                fixed_move_events.send(FixedMoveEvent::HaveWay(fixed_position, is_x));
            } else {
                fixed_move_events.send(FixedMoveEvent::NoWay);
            }
        }
    }
}
pub fn change_direction(
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
