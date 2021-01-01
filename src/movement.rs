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
    wall_pos_query: &Query<&Transform, With<Wall>>, // TODO: doesnt match destructible walls for some reaon
) -> Option<Vec2> {
    let velocity_vec = get_velocity_vec(direction, 2.0);

    let new_unit_pos = *unit_pos + velocity_vec;
    let maybe_wall = wall_pos_query.iter().find(|wall_tranform| {
        vecs_xy_intersect(new_unit_pos, wall_tranform.translation.truncate())
    });

    match maybe_wall {
        None => Some(new_unit_pos),
        Some(wall_transform) => {
            let (adjacent_cell_pos, adjacent_cell_direction) = get_adjacent_cell_entrance(
                direction,
                unit_pos,
                &wall_transform.translation.truncate(),
            );

            let has_adjacent_wall = wall_pos_query.iter().any(|wall_tranform| {
                vecs_xy_intersect(adjacent_cell_pos, wall_tranform.translation.truncate())
            });

            if has_adjacent_wall {
                None
            } else {
                Some(*unit_pos + get_velocity_vec(&adjacent_cell_direction, 2.0))
            }
        }
    }
}

fn get_adjacent_cell_entrance(
    direction: &Direction,
    unit_pos: &Vec2,
    wall_pos: &Vec2,
) -> (Vec2, Direction) {
    match direction {
        Direction::Left | Direction::Right => {
            let upper = wall_pos.y + TILE_WIDTH;
            let lower = wall_pos.y - TILE_WIDTH;

            if (upper - unit_pos.y) < (unit_pos.y - lower) {
                (Vec2::new(unit_pos.x, upper), Direction::Up)
            } else {
                (Vec2::new(unit_pos.x, lower), Direction::Down)
            }
        }
        Direction::Up | Direction::Down => {
            let right = wall_pos.x + TILE_WIDTH;
            let left = wall_pos.x - TILE_WIDTH;
            if (right - unit_pos.x) < (unit_pos.x - left) {
                (Vec2::new(right, unit_pos.y), Direction::Right)
            } else {
                (Vec2::new(left, unit_pos.y), Direction::Left)
            }
        }
    }
}

fn get_velocity_vec(direction: &Direction, speed: f32) -> Vec2 {
    match direction {
        Direction::Left => Vec2::new(-speed, 0.0),
        Direction::Up => Vec2::new(0.0, speed),
        Direction::Right => Vec2::new(speed, 0.0),
        Direction::Down => Vec2::new(0.0, -speed),
    }
}
