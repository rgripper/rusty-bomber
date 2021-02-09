use bevy::prelude::*;

use crate::{
    bomb::{BombBunble, FireBundle},
    components::{BombPower, Buff, Destructible, Direction, Fire, InGame, Portal, Wall, Way},
    constants::{FLOOR_LAYER, OBJECT_LAYER, PLAYER_LAYER},
    creatures::CreatureBundle,
    player::PlayerBundle,
    utils::{SCALE, TILE_WIDTH},
};

#[inline(always)]
fn create_sprite_sheet(
    commands: &mut Commands,
    transform: Transform,
    texture_handle: Handle<TextureAtlas>,
    index: u32,
) -> &mut Commands {
    commands
        .spawn(SpriteSheetBundle {
            transform: transform,
            texture_atlas: texture_handle,
            sprite: TextureAtlasSprite::new(index),
            ..Default::default()
        })
        .with(InGame)
}
#[inline(always)]
fn create_transform(translation: Vec2, layer: f32) -> Transform {
    Transform {
        translation: translation.extend(layer),
        scale: Vec3::splat(SCALE),
        ..Default::default()
    }
}
#[inline(always)]
fn create_transform_vec3(translation: Vec3) -> Transform {
    Transform {
        translation: translation,
        scale: Vec3::splat(SCALE),
        ..Default::default()
    }
}

#[inline(always)]
fn create_wall(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
    index: u32,
) -> &mut Commands {
    let transform = create_transform(translation, OBJECT_LAYER);
    create_sprite_sheet(commands, transform, texture_handle, index).with(Wall)
}

#[inline(always)]
fn create_way(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
    index: u32,
) {
    let transform = create_transform(translation, FLOOR_LAYER);
    create_sprite_sheet(commands, transform, texture_handle, index).with(Way);
}
#[inline(always)]
fn create_destructible_obj(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
) -> &mut Commands {
    create_yellow_way(commands, translation, texture_handle.clone());
    create_wall(commands, translation, texture_handle, 3)
}
#[inline(always)]
fn create_sprite(
    commands: &mut Commands,
    transform: Transform,
    material_handle: Handle<ColorMaterial>,
) -> &mut Commands {
    commands
        .spawn(SpriteBundle {
            material: material_handle,
            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
            transform,
            ..Default::default()
        })
        .with(InGame)
}
#[inline(always)]
fn create_buff(
    commands: &mut Commands,
    translation: Vec3,
    material_handle: Handle<ColorMaterial>,
    scale: f32,
) -> &mut Commands {
    let transform = Transform {
        translation,
        scale: Vec3::splat(scale),
        ..Default::default()
    };
    create_sprite(commands, transform, material_handle)
}
pub fn create_normal_wall(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
) {
    create_wall(commands, translation, texture_handle, 5);
}
pub fn create_last_wall(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
) {
    create_wall(commands, translation, texture_handle, 4);
}
pub fn create_green_way(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
) {
    create_way(commands, translation, texture_handle, 1)
}
pub fn create_yellow_way(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
) {
    create_way(commands, translation, texture_handle, 0)
}
pub fn create_normal_box(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
) {
    create_destructible_obj(commands, translation, texture_handle).with(Destructible::NormalBox);
}
pub fn create_player(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
    player_texture_handle: Handle<TextureAtlas>,
    player_animate_index: u32,
) -> Option<Entity> {
    create_green_way(commands, translation, texture_handle);
    let transform = create_transform(translation, PLAYER_LAYER);
    create_sprite_sheet(
        commands,
        transform,
        player_texture_handle,
        player_animate_index,
    )
    .with_bundle(PlayerBundle::default())
    .current_entity()
}
pub fn create_bomb_number_buff_box(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
) {
    create_destructible_obj(commands, translation, texture_handle)
        .with(Destructible::BombNumberBuffBox);
}
pub fn create_power_buff_box(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
) {
    create_destructible_obj(commands, translation, texture_handle).with(Destructible::PowerBuffBox);
}
pub fn create_speed_buff_box(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
) {
    create_destructible_obj(commands, translation, texture_handle).with(Destructible::SpeedBuffBox);
}
pub fn create_portal_box(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
) {
    create_destructible_obj(commands, translation, texture_handle).with(Destructible::Portal);
}
pub fn create_creature(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
    creature_texture_handle: Handle<TextureAtlas>,
) {
    create_green_way(commands, translation, texture_handle);
    let transform = create_transform(translation, PLAYER_LAYER);
    create_sprite_sheet(commands, transform, creature_texture_handle, 0)
        .with_bundle(CreatureBundle::default());
}
pub fn create_bomb(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
    player: Entity,
    power: BombPower,
) {
    let transform = create_transform(translation, OBJECT_LAYER);
    create_sprite_sheet(commands, transform, texture_handle, 0)
        .with_bundle(BombBunble::new(player, power));
}
pub fn create_ember(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
    direction: Direction,
    is_end: bool,
) {
    let index = match (direction, is_end) {
        (Direction::Left, true) => 0,
        (Direction::Right, true) => 3,
        (Direction::Up, true) => 4,
        (Direction::Down, true) => 6,
        (Direction::Left, false) | (Direction::Right, false) => 1,
        (Direction::Up, false) | (Direction::Down, false) => 5,
    };
    let transform = create_transform(translation, OBJECT_LAYER + 3.0);
    create_sprite_sheet(commands, transform, texture_handle, index).with(Fire::ember());
}
pub fn create_center_fire(
    commands: &mut Commands,
    translation: Vec2,
    texture_handle: Handle<TextureAtlas>,
    power: i32,
) {
    let transform = create_transform(translation, OBJECT_LAYER + 3.0);
    create_sprite_sheet(commands, transform, texture_handle, 8).with_bundle(FireBundle::new(power));
}

pub fn create_power_buff(
    commands: &mut Commands,
    translation: Vec3,
    material_handle: Handle<ColorMaterial>,
) {
    create_buff(commands, translation, material_handle, 1.0).with(Buff::PowerBuff);
}
pub fn create_speed_buff(
    commands: &mut Commands,
    translation: Vec3,
    material_handle: Handle<ColorMaterial>,
) {
    create_buff(commands, translation, material_handle, 0.95).with(Buff::SpeedBuff);
}
pub fn create_bomb_number_buff(
    commands: &mut Commands,
    translation: Vec3,
    material_handle: Handle<ColorMaterial>,
) {
    create_buff(commands, translation, material_handle, 1.25).with(Buff::BombNumberBuff);
}
pub fn create_portal(
    commands: &mut Commands,
    translation: Vec3,
    texture_handle: Handle<TextureAtlas>,
) {
    let transform = create_transform_vec3(translation);
    create_sprite_sheet(commands, transform, texture_handle, 1).with(Portal);
}
