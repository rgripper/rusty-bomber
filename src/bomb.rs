use crate::*;

pub const BOMB: &str = "bomb";

pub fn space_to_set_bomb(
    keyboard_input: Res<Input<KeyCode>>,
    commands: &mut Commands,
    bomb_material: Res<BombMaterial>,
    mut have_player_way_position_reader: Local<EventReader<HavePlayerWayEvent>>,
    have_player_way_position: Res<Events<HavePlayerWayEvent>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for event in have_player_way_position_reader.iter(&have_player_way_position) {
            let one = event.0;
            if event.1 {
                commands
                    .spawn(SpriteBundle {
                        material: bomb_material.0.clone(),
                        sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                        transform: Transform::from_translation(Vec3::new(
                            one.x,
                            one.y,
                            OBJECT_LAYER,
                        )),
                        ..Default::default()
                    })
                    .with(Bomb(Timer::from_seconds(3.0, false)))
                    .with(BombPower(2));
            }
        }
    }
}
pub fn bomb_trigger(
    commands: &mut Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bomb, &BombPower, &Transform)>,
    fire_material: Res<FireMaterial>,
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
            for i in 1..=power.0 {
                let i = i as f32;
                commands
                    .spawn(SpriteBundle {
                        material: fire_material.0.clone(),
                        sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                        transform: Transform::from_translation(Vec3::new(
                            translation.x + i * TILE_WIDTH,
                            translation.y,
                            OBJECT_LAYER,
                        )),
                        ..Default::default()
                    })
                    .with(Fire(Timer::from_seconds(0.5, false)));
                commands
                    .spawn(SpriteBundle {
                        material: fire_material.0.clone(),
                        sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                        transform: Transform::from_translation(Vec3::new(
                            translation.x - i * TILE_WIDTH,
                            translation.y,
                            OBJECT_LAYER,
                        )),
                        ..Default::default()
                    })
                    .with(Fire(Timer::from_seconds(0.5, false)));
                commands
                    .spawn(SpriteBundle {
                        material: fire_material.0.clone(),
                        sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                        transform: Transform::from_translation(Vec3::new(
                            translation.x,
                            translation.y + i * TILE_WIDTH,
                            OBJECT_LAYER,
                        )),
                        ..Default::default()
                    })
                    .with(Fire(Timer::from_seconds(0.5, false)));
                commands
                    .spawn(SpriteBundle {
                        material: fire_material.0.clone(),
                        sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                        transform: Transform::from_translation(Vec3::new(
                            translation.x,
                            translation.y - i * TILE_WIDTH,
                            OBJECT_LAYER,
                        )),
                        ..Default::default()
                    })
                    .with(Fire(Timer::from_seconds(0.5, false)));
            }
            commands.remove_one::<Bomb>(entity);
            commands.remove_one::<BombPower>(entity);
            commands.remove::<SpriteBundle>(entity);
        }
    }
}
pub fn remove_fire(
    commands: &mut Commands,
    time: Res<Time>,
    mut query: Query<(Entity,&mut Fire)>,
) {
    for (entity,mut fire) in query.iter_mut() {
        if fire.0.tick(time.delta_seconds()).finished() {
            commands.remove_one::<Fire>(entity);
            commands.remove::<SpriteBundle>(entity);
        }
    }
}