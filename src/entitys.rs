use bevy::prelude::*;

use crate::{
    components::{Destructible, InGame, Wall, Way},
    constants::{FLOOR_LAYER, OBJECT_LAYER, PLAYER_LAYER},
    creatures::CreatureBundle,
    player::PlayerBundle,
    utils::SCALE,
};

#[inline(always)]
fn create_sprite_sheet(
    commands: &mut Commands,
    transform: Transform,
    texture_atlas: Handle<TextureAtlas>,
    index: u32,
) -> &mut Commands {
    commands
        .spawn(SpriteSheetBundle {
            transform: transform,
            texture_atlas: texture_atlas,
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
fn create_wall(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
    index: u32,
) -> &mut Commands {
    let transform = create_transform(translation, OBJECT_LAYER);
    create_sprite_sheet(commands, transform, texture_atlas, index).with(Wall)
}

#[inline(always)]
fn create_way(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
    index: u32,
) {
    let transform = create_transform(translation, FLOOR_LAYER);
    create_sprite_sheet(commands, transform, texture_atlas, index).with(Way);
}
#[inline(always)]
fn create_destructible_obj(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
) -> &mut Commands {
    create_yellow_way(commands, translation, texture_atlas.clone());
    create_wall(commands, translation, texture_atlas, 3)
}
pub fn create_normal_wall(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
) {
    create_wall(commands, translation, texture_atlas, 5);
}
pub fn create_last_wall(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
) {
    create_wall(commands, translation, texture_atlas, 4);
}
pub fn create_green_way(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
) {
    create_way(commands, translation, texture_atlas, 1)
}
pub fn create_yellow_way(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
) {
    create_way(commands, translation, texture_atlas, 0)
}
pub fn create_normal_box(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
) {
    create_destructible_obj(commands, translation, texture_atlas).with(Destructible::NormalBox);
}
pub fn create_player(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
    player_texture_atlas: Handle<TextureAtlas>,
) -> Option<Entity> {
    create_green_way(commands, translation, texture_atlas);
    let transform = create_transform(translation, PLAYER_LAYER);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: player_texture_atlas,
            transform: transform,
            ..Default::default()
        })
        .with_bundle(PlayerBundle::default())
        .with(InGame)
        .current_entity()
}
pub fn create_bomb_number_buff_box(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
) {
    create_destructible_obj(commands, translation, texture_atlas)
        .with(Destructible::BombNumberBuffBox);
}
pub fn create_power_buff_box(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
) {
    create_destructible_obj(commands, translation, texture_atlas).with(Destructible::PowerBuffBox);
}
pub fn create_speed_buff_box(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
) {
    create_destructible_obj(commands, translation, texture_atlas).with(Destructible::SpeedBuffBox);
}
pub fn create_portal_box(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
) {
    create_destructible_obj(commands, translation, texture_atlas).with(Destructible::Portal);
}
pub fn create_creature(
    commands: &mut Commands,
    translation: Vec2,
    texture_atlas: Handle<TextureAtlas>,
    creature_texture_atlas: Handle<TextureAtlas>,
) {
    create_green_way(commands, translation, texture_atlas);
    let transform = create_transform(translation, PLAYER_LAYER);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: creature_texture_atlas,
            transform: transform,
            ..Default::default()
        })
        .with_bundle(CreatureBundle::default())
        .with(InGame);
}
