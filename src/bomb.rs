use bevy::prelude::*;
use bevy_rapier2d::{
    physics::{ColliderHandleComponent, RigidBodyHandleComponent},
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
};

use crate::{
    assets::{
        BombNumberBuffMaterial, BombTextureAtlas, FireTextureAtlas, PortalTextureAtlas,
        PowerBuffMaterial, SpeedBuffMaterial,
    },
    components::{
        Animation, Bomb, BombNumber, BombPower, Destructible, Direction, Ember, Fire, Player, Stop,
        Wall, FIRE_ANIMATE_TIME,
    },
    entitys::{
        create_bomb, create_bomb_number_buff, create_center_fire, create_collider, create_ember,
        create_portal, create_power_buff, create_speed_buff, create_static_rigid_body,
    },
    events::GameEvents,
    resources::Map,
    state::RunState,
    utils::{aabb_detection, TILE_WIDTH},
};

pub trait BombSystems {
    fn bomb_systems(&mut self) -> &mut Self;
}
impl BombSystems for SystemStage {
    fn bomb_systems(&mut self) -> &mut Self {
        self.add_system(space_to_set_bomb.system())
            .add_system(bomb_trigger.system())
            .add_system(despawn_fire.system())
            .add_system(bomb_block_player.system())
            .add_system(bomb_destruction.system())
            .add_system(animate_bomb.system())
            .add_system(animate_fire.system())
            .add_system(ember_trigger.system())
            .add_system(for_wall_add_collision_detection.system())
    }
}

#[derive(Bundle)]
pub struct BombBunble {
    bomb: Bomb,
    power: BombPower,
    animate: Animation,
}
impl BombBunble {
    pub fn new(player: Entity, power: BombPower) -> Self {
        Self {
            bomb: Bomb {
                player,
                ..Default::default()
            },
            power: power,
            animate: Animation(Timer::from_seconds(1.0, true)),
        }
    }
}
fn for_wall_add_collision_detection(
    commands: &mut Commands,
    query: Query<
        (Entity, &Transform),
        (
            With<Wall>,
            Without<RigidBodyBuilder>,
            Without<ColliderBuilder>,
            Without<RigidBodyHandleComponent>,
            Without<ColliderHandleComponent>,
        ),
    >,
) {
    for (entity, transform) in query.iter() {
        let translation = transform.translation;
        commands.insert(
            entity,
            (
                create_static_rigid_body(translation.x, translation.y),
                create_collider(entity),
            ),
        );
    }
}
fn space_to_set_bomb(
    commands: &mut Commands,
    bomb_texture_atlas: Res<BombTextureAtlas>,
    runstate: Res<RunState>,
    keyboard_input: Res<Input<KeyCode>>,
    bomb_position: Query<&Transform, With<Bomb>>,
    mut player_query: Query<
        (&Transform, &BombPower, &mut BombNumber),
        (With<Player>, Without<Stop>),
    >,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Some(entity) = runstate.player {
            for (transform, &power, mut number) in player_query.iter_mut() {
                let position = transform.translation;
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
                let one = Vec2::new(handle(number_x), handle(number_y));

                let mut is_not_exist = true;
                'bomb: for bomb_position in bomb_position.iter() {
                    if bomb_position.translation.truncate() == one {
                        is_not_exist = false;
                        break 'bomb;
                    }
                }
                if is_not_exist && number.is_enough() {
                    create_bomb(commands, one, bomb_texture_atlas.0.clone(), entity, power);
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
#[derive(Bundle)]
pub struct FireBundle {
    fire: Fire,
    animation: Animation,
    ember: Ember,
}
impl FireBundle {
    pub fn new(power: i32) -> Self {
        Self {
            fire: Fire::default(),
            animation: Animation(Timer::from_seconds(FIRE_ANIMATE_TIME, true)),
            ember: Ember::new(power),
        }
    }
}
fn bomb_trigger(
    commands: &mut Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bomb, &BombPower, &Transform)>,
    fire_texture_atlas: Res<FireTextureAtlas>,
    mut recovery_bomb_number_events: ResMut<Events<GameEvents>>,
) {
    for (entity, mut bomb, power, transform) in query.iter_mut() {
        let translation = transform.translation;
        if bomb.timer.tick(time.delta_seconds()).finished() {
            create_center_fire(
                commands,
                translation.truncate(),
                fire_texture_atlas.0.clone(),
                power.0,
            );
            commands.despawn(entity);
            recovery_bomb_number_events.send(GameEvents::RecoveryBombNumber(bomb.player));
        }
    }
}

fn ember_trigger(
    commands: &mut Commands,
    time: Res<Time>,
    map: Res<Map>,
    fire_texture_atlas: Res<FireTextureAtlas>,
    mut fire_query: Query<(&Transform, &mut Ember), With<Fire>>,
) {
    for (transform, mut ember) in fire_query.iter_mut() {
        let power = ember.1;
        let translation = transform.translation;
        if ember.0.tick(time.delta_seconds()).just_finished() {
            let (mut up, mut down, mut left, mut right) = (true, true, true, true);
            for i in 1..=power {
                let i = i as f32;
                if up {
                    let position = Vec2::new(translation.x, translation.y + i * TILE_WIDTH);
                    // 9 and 1
                    let x = position.x / TILE_WIDTH;
                    let y = map.len() as f32 - (position.y / TILE_WIDTH + 1.0);
                    if let Some(rows) = map.get(y as usize) {
                        if let Some(&value) = rows.get(x as usize) {
                            if value == 9 || value == 1 {
                                up = false;
                            }
                        } else {
                            up = false;
                        }
                    } else {
                        up = false;
                    }
                    if up {
                        create_ember(
                            commands,
                            position,
                            fire_texture_atlas.0.clone(),
                            Direction::Up,
                            i == (power as f32),
                        );
                    }
                }

                if down {
                    let position = Vec2::new(translation.x, translation.y - i * TILE_WIDTH);
                    // 9 and 1
                    let x = position.x / TILE_WIDTH;
                    let y = map.len() as f32 - (position.y / TILE_WIDTH + 1.0);
                    if let Some(rows) = map.get(y as usize) {
                        if let Some(&value) = rows.get(x as usize) {
                            if value == 9 || value == 1 {
                                down = false;
                            }
                        } else {
                            down = false;
                        }
                    } else {
                        down = false;
                    }
                    if down {
                        create_ember(
                            commands,
                            position,
                            fire_texture_atlas.0.clone(),
                            Direction::Down,
                            i == (power as f32),
                        );
                    }
                }

                if left {
                    let position = Vec2::new(translation.x - i * TILE_WIDTH, translation.y);
                    // 9 and 1
                    let x = position.x / TILE_WIDTH;
                    let y = map.len() as f32 - (position.y / TILE_WIDTH + 1.0);
                    if let Some(rows) = map.get(y as usize) {
                        if let Some(&value) = rows.get(x as usize) {
                            if value == 9 || value == 1 {
                                left = false;
                            }
                        } else {
                            left = false;
                        }
                    } else {
                        left = false;
                    }

                    if left {
                        create_ember(
                            commands,
                            position,
                            fire_texture_atlas.0.clone(),
                            Direction::Left,
                            i == (power as f32),
                        );
                    }
                }

                if right {
                    let position = Vec2::new(translation.x + i * TILE_WIDTH, translation.y);
                    // 9 and 1
                    let x = position.x / TILE_WIDTH;
                    let y = map.len() as f32 - (position.y / TILE_WIDTH + 1.0);
                    if let Some(rows) = map.get(y as usize) {
                        if let Some(&value) = rows.get(x as usize) {
                            if value == 9 || value == 1 {
                                right = false;
                            }
                        } else {
                            right = false;
                        }
                    } else {
                        right = false;
                    }
                    if right {
                        create_ember(
                            commands,
                            position,
                            fire_texture_atlas.0.clone(),
                            Direction::Right,
                            i == (power as f32),
                        );
                    }
                }
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

fn bomb_destruction(
    commands: &mut Commands,
    destructable_wall_query: Query<(Entity, &Transform, &Destructible), With<Destructible>>,
    fire_query: Query<&Transform, With<Fire>>,
    power_buff_material: Res<PowerBuffMaterial>,
    speed_buff_material: Res<SpeedBuffMaterial>,
    portal_texture_atlas: Res<PortalTextureAtlas>,
    bomb_number_buff_material: Res<BombNumberBuffMaterial>,
    mut game_over_events: ResMut<Events<GameEvents>>,
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
                Destructible::NormalBox => {
                    commands.despawn(entity);
                }
                Destructible::PowerBuffBox => {
                    commands.despawn(entity);
                    create_power_buff(commands, position, power_buff_material.0.clone());
                }
                Destructible::SpeedBuffBox => {
                    commands.despawn(entity);
                    create_speed_buff(commands, position, speed_buff_material.0.clone());
                }
                Destructible::BombNumberBuffBox => {
                    commands.despawn(entity);
                    create_bomb_number_buff(
                        commands,
                        position,
                        bomb_number_buff_material.0.clone(),
                    );
                }
                Destructible::Portal => {
                    commands.despawn(entity);
                    create_portal(commands, position, portal_texture_atlas.0.clone());
                }
                Destructible::Player => {
                    commands.despawn(entity);

                    game_over_events.send(GameEvents::GameOver);
                }
                Destructible::Creature => {
                    commands.despawn(entity);
                    info!("Destroy a creature!");
                }
            }
        }
    }
}
