use crate::{
    components::{
        Animation, BombNumber, BombPower, Direction, Player, PlayerAnimation, PlayerPosition,
        Velocity, Wall,
    },
    constants::PLAYER_LAYER,
    utils::{vecs_xy_intersect, TILE_WIDTH},
};

use bevy::prelude::*;

pub trait PlayerSystems {
    fn player_systems(&mut self) -> &mut Self;
}
impl PlayerSystems for SystemStage {
    fn player_systems(&mut self) -> &mut Self {
        self
            // movement
            .add_system(player_movement.system())
            .add_system(change_direction.system())
            .add_system(position_to_translation.system())
            // animate
            .add_system(animate_player.system())
            .add_system(velocity_to_animation.system())
    }
}
#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    direction: Direction,
    velocity: Velocity,
    bomb_power: BombPower,
    bomb_number: BombNumber,
    animation: Animation,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            direction: Direction::Right,
            velocity: Velocity {
                current: 0.0,
                max: 2.0,
            },
            bomb_power: BombPower(1),
            bomb_number: BombNumber { max: 1, current: 0 },
            animation: Animation(Timer::from_seconds(1.0, true)),
        }
    }
}

// movement
fn position_to_translation(
    mut query: Query<(&PlayerPosition, &mut Transform), Changed<PlayerPosition>>,
) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(position.x, position.y, position.z);
    }
}
pub fn player_movement(
    mut query: Query<(&Velocity, &Direction, &mut PlayerPosition), With<Player>>,
    wall_pos_query: Query<&Transform, With<Wall>>,
) {
    for (_, direction, mut player_position) in query
        .iter_mut()
        .filter(|(velocity, _, _)| velocity.current > 0.0)
    {
        if let Some(new_pos) = move_or_turn(&player_position.truncate(), direction, &wall_pos_query)
        {
            player_position.0 = new_pos.extend(PLAYER_LAYER);
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
            //println!("down");
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

pub fn move_or_turn(
    unit_pos: &Vec2,
    direction: &Direction,
    wall_pos_query: &Query<&Transform, With<Wall>>, // TODO: doesnt match destructible walls for some reaon
) -> Option<Vec2> {
    let velocity_vec = get_velocity_vec(direction, 2.0);

    let new_unit_pos = *unit_pos + velocity_vec;
    let maybe_wall = wall_pos_query.iter().find(|wall_tranform| {
        vecs_xy_intersect(&new_unit_pos, &wall_tranform.translation.truncate())
    });

    match maybe_wall {
        None => Some(new_unit_pos),
        Some(wall_transform) => {
            let (turn_point_1, turn_point_2, adjacent_cell_direction) =
                get_turn_waypoints(direction, unit_pos, &wall_transform.translation.truncate());

            let has_free_waypoints = wall_pos_query.iter().all(|other_wall_tranform| {
                let other_wall_pos = &other_wall_tranform.translation.truncate();
                !vecs_xy_intersect(&turn_point_1, other_wall_pos)
                    && !vecs_xy_intersect(&turn_point_2, other_wall_pos)
            });

            if has_free_waypoints {
                Some(*unit_pos + get_velocity_vec(&adjacent_cell_direction, 2.0))
            } else {
                None
            }
        }
    }
}

fn get_turn_waypoints(
    direction: &Direction,
    unit_pos: &Vec2,
    wall_pos: &Vec2,
) -> (Vec2, Vec2, Direction) {
    match direction {
        Direction::Left | Direction::Right => {
            let upper = wall_pos.y + TILE_WIDTH;
            let lower = wall_pos.y - TILE_WIDTH;

            if (upper - unit_pos.y) < (unit_pos.y - lower) {
                (
                    Vec2::new(unit_pos.x, upper),
                    Vec2::new(wall_pos.x, upper),
                    Direction::Up,
                )
            } else {
                (
                    Vec2::new(unit_pos.x, lower),
                    Vec2::new(wall_pos.x, lower),
                    Direction::Down,
                )
            }
        }
        Direction::Up | Direction::Down => {
            let right = wall_pos.x + TILE_WIDTH;
            let left = wall_pos.x - TILE_WIDTH;
            if (right - unit_pos.x) < (unit_pos.x - left) {
                (
                    Vec2::new(right, unit_pos.y),
                    Vec2::new(right, wall_pos.y),
                    Direction::Right,
                )
            } else {
                (
                    Vec2::new(left, unit_pos.y),
                    Vec2::new(left, wall_pos.y),
                    Direction::Left,
                )
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

// animate
fn animate_player(
    time: Res<Time>,
    // texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Animation,
        &mut TextureAtlasSprite,
        &Velocity,
        &Direction,
    )>,
) {
    for (mut animation, mut sprite, _, direction) in query
        .iter_mut()
        .filter(|(_, _, velocity, _)| velocity.current != 0.0)
    {
        let indexs = PlayerAnimation::from(*direction).indexs;
        let mut should_turn = true;
        'contatine: for &idx in indexs.iter() {
            if sprite.index == idx {
                should_turn = false;
                break 'contatine;
            }
        }
        if should_turn {
            sprite.index = indexs[0];
        }
        animation.0.tick(time.delta_seconds());
        if animation.0.just_finished() {
            let indexs = PlayerAnimation::from(*direction).indexs;
            if sprite.index == indexs[0] {
                sprite.index = indexs[1];
            } else if sprite.index == indexs[1] {
                sprite.index = indexs[2];
            } else {
                sprite.index = indexs[0];
            }
            //info!("index:{}", sprite.index);
        }
    }
}
fn velocity_to_animation(
    mut query: Query<(&Velocity, &mut Animation), (With<Player>, Changed<Velocity>)>,
) {
    for (velocity, mut animation) in query.iter_mut() {
        animation.0.set_duration(1.0 / velocity.max * 0.5);
    }
}
