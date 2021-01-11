use bevy::prelude::*;

use crate::{
    assets::{
        BombNumberBuffMaterial, BombTextureAtlas, FireTextureAtlas, PortalTextureAtlas,
        PowerBuffMaterial, SpeedBuffMaterial,
    },
    components::{
        Animation, Bomb, BombNumber, BombPower, Buff, Destructable, Ember, Fire, InGame, Player,
        PlayerPosition, Portal, Stop, Wall, FIRE_ANIMATE_TIME,
    },
    constants::OBJECT_LAYER,
    creatures::Creature,
    events::{GameOverEvent, GameOverType, RecoveryBombNumberEvent},
    state::RunState,
    utils::{aabb_detection, vecs_xy_intersect, SCALE, TILE_WIDTH},
};

pub const BOMB: &str = "bomb";
pub trait BombSystems {
    fn bomb_systems(&mut self) -> &mut Self;
}
impl BombSystems for SystemStage {
    fn bomb_systems(&mut self) -> &mut Self {
        self.add_system(space_to_set_bomb.system())
            .add_system(bomb_trigger.system())
            .add_system(recovery_bomb_number.system())
            .add_system(despawn_fire.system())
            .add_system(bomb_block_player.system())
            .add_system(bomb_destruction.system())
            .add_system(bomb_player.system())
            .add_system(bomb_creature.system())
            .add_system(animate_bomb.system())
            .add_system(animate_fire.system())
            .add_system(ember_trigger.system())
    }
}

fn space_to_set_bomb(
    commands: &mut Commands,
    bomb_texture_atlas: Res<BombTextureAtlas>,
    runstate: Res<RunState>,
    keyboard_input: Res<Input<KeyCode>>,
    bomb_position: Query<&Transform, With<Bomb>>,
    mut player_query: Query<
        (&PlayerPosition, &BombPower, &mut BombNumber),
        (With<Player>, Without<Stop>),
    >,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(entity) = runstate.player {
            for (position, power, mut number) in player_query.iter_mut() {
                fn handle(n: f32) -> f32 {
                    let a = n.floor();
                    let b = n.fract();
                    if b >= 0.5 {
                        (a + 1.0) * TILE_WIDTH
                    } else {
                        a * TILE_WIDTH
                    }
                }
                let number_x = position.x / TILE_WIDTH;
                let number_y = position.y / TILE_WIDTH;
                let one = Vec3::new(handle(number_x), handle(number_y), OBJECT_LAYER);

                let mut is_not_exist = true;
                'bomb: for bomb_position in bomb_position.iter() {
                    if bomb_position.translation == one {
                        is_not_exist = false;
                        break 'bomb;
                    }
                }
                if is_not_exist && number.is_enough() {
                    let bomb_transform = Transform {
                        translation: one,
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    };
                    commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: bomb_texture_atlas.0.clone(),
                            transform: bomb_transform,
                            ..Default::default()
                        })
                        .with(Bomb {
                            timer: Timer::from_seconds(3.0, false),
                            player: entity,
                        })
                        .with(BombPower(power.0))
                        .with(Animation(Timer::from_seconds(1.0, true)))
                        .with(InGame);
                    number.current += 1;
                }
            }
        }
    }
}
fn animate_bomb(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite), With<Bomb>>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        // info!("index:{}",sprite.index);
        animation.0.tick(time.delta_seconds());
        if animation.0.just_finished() {
            if sprite.index == 0 {
                sprite.index = 1;
            } else if sprite.index == 1 {
                sprite.index = 2;
            } else {
                sprite.index = 0;
            }
        }
    }
}
fn animate_fire(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite), With<Fire>>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        //info!("index:{}", sprite.index);
        // 9 10 11 3
        animation.0.tick(time.delta_seconds());
        if animation.0.just_finished() {
            if sprite.index == 8 {
                sprite.index = 9;
            } else if sprite.index == 9 {
                sprite.index = 10;
            } else if sprite.index == 10 {
                sprite.index = 2;
            }
        }
    }
}

fn bomb_trigger(
    commands: &mut Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bomb, &BombPower, &Transform)>,
    fire_texture_atlas: Res<FireTextureAtlas>,
    mut recovery_bomb_number_events: ResMut<Events<RecoveryBombNumberEvent>>,
) {
    for (entity, mut bomb, power, transform) in query.iter_mut() {
        let translation = transform.translation;
        if bomb.timer.tick(time.delta_seconds()).finished() {
            let fire_transform = Transform {
                translation: Vec3::new(translation.x, translation.y, OBJECT_LAYER),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            };
            commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: fire_texture_atlas.0.clone(),
                    transform: fire_transform,
                    sprite: TextureAtlasSprite::new(8),
                    ..Default::default()
                })
                .with(Fire::default())
                .with(Animation(Timer::from_seconds(FIRE_ANIMATE_TIME, true)))
                .with(Ember::new(power.0))
                .with(InGame);

            commands.despawn(entity);
            recovery_bomb_number_events.send(RecoveryBombNumberEvent(bomb.player));
        }
    }
}
fn ember_trigger(
    commands: &mut Commands,
    time: Res<Time>,
    fire_texture_atlas: Res<FireTextureAtlas>,
    mut fire_query: Query<(&Transform, &mut Ember)>,
    wall_query: Query<&Transform, (With<Wall>, Without<Bomb>, Without<Destructable>)>,
) {
    for (transform, mut ember) in fire_query.iter_mut() {
        let power = ember.1;
        let translation = transform.translation;
        if ember.0.tick(time.delta_seconds()).just_finished() {
            let (mut up, mut down, mut left, mut right) = (true, true, true, true);
            for i in 1..=power {
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
                        let index = if i == (power as f32) { 4 } else { 5 };
                        let ember_transform = Transform {
                            translation: position,
                            scale: Vec3::splat(SCALE),
                            ..Default::default()
                        };
                        commands
                            .spawn(SpriteSheetBundle {
                                texture_atlas: fire_texture_atlas.0.clone(),
                                sprite: TextureAtlasSprite::new(index),
                                transform: ember_transform,
                                ..Default::default()
                            })
                            .with(Fire::ember())
                            .with(InGame);
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
                        let index = if i == (power as f32) { 6 } else { 5 };
                        let ember_transform = Transform {
                            translation: position,
                            scale: Vec3::splat(SCALE),
                            ..Default::default()
                        };
                        commands
                            .spawn(SpriteSheetBundle {
                                texture_atlas: fire_texture_atlas.0.clone(),
                                sprite: TextureAtlasSprite::new(index),
                                transform: ember_transform,
                                ..Default::default()
                            })
                            .with(Fire::ember())
                            .with(InGame);
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
                        let index = if i == (power as f32) { 0 } else { 1 };
                        let ember_transform = Transform {
                            translation: position,
                            scale: Vec3::splat(SCALE),
                            ..Default::default()
                        };
                        commands
                            .spawn(SpriteSheetBundle {
                                texture_atlas: fire_texture_atlas.0.clone(),
                                sprite: TextureAtlasSprite::new(index),
                                transform: ember_transform,
                                ..Default::default()
                            })
                            .with(Fire::ember())
                            .with(InGame);
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
                        let index = if i == (power as f32) { 3 } else { 1 };
                        let ember_transform = Transform {
                            translation: position,
                            scale: Vec3::splat(SCALE),
                            ..Default::default()
                        };
                        commands
                            .spawn(SpriteSheetBundle {
                                texture_atlas: fire_texture_atlas.0.clone(),
                                sprite: TextureAtlasSprite::new(index),
                                transform: ember_transform,
                                ..Default::default()
                            })
                            .with(Fire::ember())
                            .with(InGame);
                    }
                }
            }
        }
    }
}
fn recovery_bomb_number(
    recovery_bomb_number_events: Res<Events<RecoveryBombNumberEvent>>,
    mut events_reader: Local<EventReader<RecoveryBombNumberEvent>>,
    mut player_query: Query<(Entity, &mut BombNumber), With<Player>>,
) {
    for entity in events_reader.iter(&recovery_bomb_number_events) {
        let entity = entity.0;
        'bomb_number: for (player, mut number) in player_query.iter_mut() {
            if entity == player {
                number.current -= 1;
                // info!("current:{}", number.current);
                // info!("max:{}", number.max);
                break 'bomb_number;
            }
        }
    }
}

fn despawn_fire(commands: &mut Commands, time: Res<Time>, mut query: Query<(Entity, &mut Fire)>) {
    for (entity, mut fire) in query.iter_mut() {
        if fire.0.tick(time.delta_seconds()).finished() {
            commands.despawn(entity);
        }
    }
}
fn bomb_block_player(
    commands: &mut Commands,
    bomb_query: Query<(Entity, &Transform), (With<Bomb>, Without<Wall>)>,
    player_query: Query<&Transform, With<Player>>,
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

// TODO: refactor to be `bomb_unit`
fn bomb_player(
    commands: &mut Commands,
    query: Query<(Entity, &PlayerPosition)>,
    runstate: Res<RunState>,
    mut game_over_events: ResMut<Events<GameOverEvent>>,
    fire_query: Query<&Transform, With<Fire>>,
) {
    let mut should_send_game_over = false;
    for (entity, position) in query.iter() {
        let mut need_destroy = false;
        'fire: for fire in fire_query.iter() {
            if aabb_detection(fire.translation.x, fire.translation.y, position.0) {
                need_destroy = true;
                break 'fire;
            }
        }
        if need_destroy {
            if let Some(player) = runstate.player {
                if player == entity {
                    should_send_game_over = true;
                }
            }
            commands.despawn(entity);
        }
    }
    if should_send_game_over {
        game_over_events.send(GameOverEvent(GameOverType::Defeat));
    }
}

// TODO: refactor to be `bomb_unit`
fn bomb_creature(
    commands: &mut Commands,
    query: Query<(Entity, &Transform), With<Creature>>,
    fire_query: Query<&Transform, With<Fire>>,
) {
    for (entity, position) in query.iter() {
        if fire_query.iter().any(|fire| {
            vecs_xy_intersect(
                &fire.translation.truncate(),
                &position.translation.truncate(),
            )
        }) {
            commands.despawn(entity);
        }
    }
}

fn bomb_destruction(
    commands: &mut Commands,
    destructable_wall_query: Query<(Entity, &Transform, &Destructable), With<Destructable>>,
    fire_query: Query<&Transform, With<Fire>>,
    power_buff_material: Res<PowerBuffMaterial>,
    speed_buff_material: Res<SpeedBuffMaterial>,
    portal_texture_atlas: Res<PortalTextureAtlas>,
    bomb_number_buff_material: Res<BombNumberBuffMaterial>,
) {
    for (entity, transform, destructable) in destructable_wall_query.iter() {
        let position = transform.translation;
        let mut need_destroy = false;
        'fire: for fire in fire_query.iter() {
            if aabb_detection(fire.translation.x, fire.translation.y, position) {
                need_destroy = true;
                break 'fire;
            }
        }

        if need_destroy {
            match destructable {
                Destructable::NormalBox => {
                    commands.despawn(entity);
                }
                Destructable::PowerBuffBox => {
                    commands.despawn(entity);
                    commands
                        .spawn(SpriteBundle {
                            material: power_buff_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(position),
                            ..Default::default()
                        })
                        .with(Buff::PowerBuff)
                        .with(InGame);
                }
                Destructable::SpeedBuffBox => {
                    commands.despawn(entity);
                    commands
                        .spawn(SpriteBundle {
                            material: speed_buff_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(position),
                            ..Default::default()
                        })
                        .with(Buff::SpeedBuff)
                        .with(InGame);
                }
                Destructable::BombNumberBuffBox => {
                    commands.despawn(entity);
                    commands
                        .spawn(SpriteBundle {
                            material: bomb_number_buff_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(position),
                            ..Default::default()
                        })
                        .with(Buff::BombNumberBuff)
                        .with(InGame);
                }
                Destructable::Portal => {
                    commands.despawn(entity);

                    commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: portal_texture_atlas.0.clone(),
                            transform: Transform {
                                translation: position,
                                scale: Vec3::splat(SCALE),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with(Portal)
                        .with(InGame);
                }
            }
        }
    }
}
