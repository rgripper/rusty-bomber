use crate::Direction;
use crate::*;

pub const MOVEMENT: &str = "movement";

pub fn player_movement(
    mut query: Query<(&Velocity, &Direction, &mut Transform), (With<Player>, Changed<Velocity>)>,
    wall_pos_query: Query<&Transform, With<Wall>>,
) {
    for (_, direction, mut unit_transform) in query
        .iter_mut()
        .filter(|(velocity, _, _)| velocity.current > 0.0)
    {
        if let Some(new_pos) = move_or_turn(
            &unit_transform.translation.truncate(),
            direction,
            &wall_pos_query,
        ) {
            unit_transform.translation = new_pos.extend(unit_transform.translation.z);
            // conversion from Vec2 to Vec3
        }
    }
}

fn fix_player_translation(
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
    mut query: Query<(&mut Direction, &mut Velocity), With<Player>>,
) {
    for (mut direction, mut velocity) in query.iter_mut() {
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
            velocity.current = velocity.max;
        } else {
            velocity.current = 0.0;
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
