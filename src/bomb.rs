use crate::*;

pub const BOMB: &str = "bomb";

pub fn space_to_set_bomb(
    keyboard_input: Res<Input<KeyCode>>,
    mut request_spawn_bomb_event: ResMut<Events<RequestSpawnBoomEvent>>,
    have_player_way_position: Res<Events<HavePlayerWayEvent>>,
    bomb_position: Query<&Transform, (With<Bomb>)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for event in have_player_way_position
            .get_reader()
            .iter(&have_player_way_position)
        {
            let one = Vec3::new(event.0.x, event.0.y, OBJECT_LAYER);
            let is_way = event.1;
            let mut is_exist = true;
            for bomb_position in bomb_position.iter() {
                is_exist = bomb_position.translation != one;
            }

            if is_way && is_exist {
                request_spawn_bomb_event.send(RequestSpawnBoomEvent(one));
            }
        }
    }
}
pub fn request_spawn_bomb(
    commands: &mut Commands,
    bomb_material: Res<BombMaterial>,
    request_spawn_bomb_event: Res<Events<RequestSpawnBoomEvent>>,
) {
    for is_ok in request_spawn_bomb_event
        .get_reader()
        .iter(&request_spawn_bomb_event)
    {
        commands
            .spawn(SpriteBundle {
                material: bomb_material.0.clone(),
                sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                transform: Transform::from_translation(is_ok.0),
                ..Default::default()
            })
            .with(Bomb(Timer::from_seconds(3.0, false)))
            .with(BombPower(2));
    }
}
pub fn bomb_trigger(
    commands: &mut Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bomb, &BombPower, &Transform)>,
    fire_material: Res<FireMaterial>,
    wall_query: Query<&Transform, (With<Wall>, Without<Bomb>, Without<Destructable>)>,
) {
    for (entity, mut bomb, power, transform) in query.iter_mut() {
        let translation = transform.translation;
        if bomb.0.tick(time.delta_seconds()).finished() {
            commands
                .spawn(SpriteBundle {
                    material: fire_material.0.clone(),
                    sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                    transform: Transform::from_translation(Vec3::new(
                        translation.x,
                        translation.y,
                        OBJECT_LAYER,
                    )),
                    ..Default::default()
                })
                .with(Fire(Timer::from_seconds(0.5, false)));

            let (mut up, mut down, mut left, mut right) = (true, true, true, true);
            for i in 1..=power.0 {
                let i = i as f32;
                if up {
                    let position =
                        Vec3::new(translation.x, translation.y + i * TILE_WIDTH, OBJECT_LAYER);
                    for wall in wall_query.iter() {
                        if aabb_detection(wall.translation.x, wall.translation.y, position) {
                            up = false;
                        }
                    }
                    if up {
                        commands
                            .spawn(SpriteBundle {
                                material: fire_material.0.clone(),
                                sprite: Sprite::new(Vec2::new(
                                    TILE_WIDTH as f32,
                                    TILE_WIDTH as f32,
                                )),
                                transform: Transform::from_translation(position),
                                ..Default::default()
                            })
                            .with(Fire(Timer::from_seconds(0.5, false)));
                    }
                }

                if down {
                    let position =
                        Vec3::new(translation.x, translation.y - i * TILE_WIDTH, OBJECT_LAYER);
                    for wall in wall_query.iter() {
                        if aabb_detection(wall.translation.x, wall.translation.y, position) {
                            down = false;
                        }
                    }
                    if down {
                        commands
                            .spawn(SpriteBundle {
                                material: fire_material.0.clone(),
                                sprite: Sprite::new(Vec2::new(
                                    TILE_WIDTH as f32,
                                    TILE_WIDTH as f32,
                                )),
                                transform: Transform::from_translation(position),
                                ..Default::default()
                            })
                            .with(Fire(Timer::from_seconds(0.5, false)));
                    }
                }

                if left {
                    let position =
                        Vec3::new(translation.x - i * TILE_WIDTH, translation.y, OBJECT_LAYER);
                    for wall in wall_query.iter() {
                        if aabb_detection(wall.translation.x, wall.translation.y, position) {
                            left = false;
                        }
                    }
                    if left {
                        commands
                            .spawn(SpriteBundle {
                                material: fire_material.0.clone(),
                                sprite: Sprite::new(Vec2::new(
                                    TILE_WIDTH as f32,
                                    TILE_WIDTH as f32,
                                )),
                                transform: Transform::from_translation(position),
                                ..Default::default()
                            })
                            .with(Fire(Timer::from_seconds(0.5, false)));
                    }
                }

                if right {
                    let position =
                        Vec3::new(translation.x + i * TILE_WIDTH, translation.y, OBJECT_LAYER);
                    for wall in wall_query.iter() {
                        if aabb_detection(wall.translation.x, wall.translation.y, position) {
                            right = false;
                        }
                    }
                    if right {
                        commands
                            .spawn(SpriteBundle {
                                material: fire_material.0.clone(),
                                sprite: Sprite::new(Vec2::new(
                                    TILE_WIDTH as f32,
                                    TILE_WIDTH as f32,
                                )),
                                transform: Transform::from_translation(position),
                                ..Default::default()
                            })
                            .with(Fire(Timer::from_seconds(0.5, false)));
                    }
                }
            }
            commands.despawn(entity);
        }
    }
}

pub fn despawn_fire(
    commands: &mut Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Fire)>,
) {
    for (entity, mut fire) in query.iter_mut() {
        if fire.0.tick(time.delta_seconds()).finished() {
            commands.despawn(entity);
        }
    }
}
pub fn bomb_block_player(
    commands: &mut Commands,
    bomb_query: Query<(Entity, &Transform), (With<Bomb>, Without<Wall>)>,
    player_query: Query<&Transform, (With<Player>)>,
) {
    for (entity, bomb_position) in bomb_query.iter() {
        for player_position in player_query.iter() {
            let x = player_position.translation.x;
            let y = player_position.translation.y;
            if !aabb_detection(x, y, bomb_position.translation) {
                commands.insert_one(entity, Wall);
            }
        }
    }
}
pub fn bomb_destruction(
    commands: &mut Commands,
    destructable_wall_query: Query<(Entity, &Transform), (With<Destructable>)>,
    fire_query: Query<&Transform, (With<Fire>)>,
) {
    for (entity, transform) in destructable_wall_query.iter() {
        let position = transform.translation;
        let mut need_destroy = false;
        'fire: for fire in fire_query.iter() {
            if aabb_detection(fire.translation.x, fire.translation.y, position) {
                need_destroy = true;
                break 'fire;
            }
        }
        if need_destroy {
            commands.despawn(entity);
        }
    }
}
pub fn bomb_life_things(
    mut query: Query<(&Transform, &mut Life), (With<Life>)>,
    fire_query: Query<&Transform, (With<Fire>)>,
) {
    for (position, mut life) in query.iter_mut() {
        let position = position.translation;
        let mut need_hurt = false;
        'fire: for fire in fire_query.iter() {
            if aabb_detection(fire.translation.x, fire.translation.y, position) {
                need_hurt = true;
                break 'fire;
            }
        }
        if need_hurt {
            life.update(-1);
        }
    }
}
