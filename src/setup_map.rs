use crate::{
    assets::*,
    bundle::PlayerBundle,
    components::{Destructable, InGame, PlayerPosition, Wall, Way},
    constants::{FLOOR_LAYER, OBJECT_LAYER, PLAYER_LAYER},
    resources::Map,
    state::RunState,
    utils::TILE_WIDTH,
};
use bevy::prelude::*;

pub const GMAE_SETUP: &str = "game_setup";

pub fn setup_map(
    commands: &mut Commands,
    perma_wall_material: Res<PermaWallMaterial>,
    map_resource: Res<Map>,
    destructable_wall_material: Res<DestructableWallMaterial>,
    player_texture_atlas: Res<PlayerTextureAtlas>,
    floor_material: Res<FloorMaterial>,
    mut runstate: ResMut<RunState>,
) {
    let room_map = map_resource.map_value();

    for (row_index, row) in room_map.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            // Using match here makes it easier to extend the map
            match *cell {
                1 => {
                    commands
                        .spawn(SpriteBundle {
                            material: perma_wall_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                OBJECT_LAYER,
                            )),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(InGame);
                }

                2 => {
                    commands
                        .spawn(SpriteBundle {
                            material: floor_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                FLOOR_LAYER,
                            )),
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);
                    commands
                        .spawn(SpriteBundle {
                            material: destructable_wall_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                OBJECT_LAYER,
                            )),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(Destructable::NormalBox)
                        .with(InGame);
                }
                // When setting each level, the player’s position should be set flexibly
                3 => {
                    // way
                    commands
                        .spawn(SpriteBundle {
                            material: floor_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                FLOOR_LAYER,
                            )),
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);

                    // player
                    let player = commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: player_texture_atlas.0.clone(),
                            ..Default::default()
                        })
                        .with_bundle(PlayerBundle::default())
                        .with(PlayerPosition::from(Vec3::new(
                            TILE_WIDTH * col_index as f32,
                            TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                            PLAYER_LAYER,
                        )))
                        .with(InGame)
                        .current_entity();
                    runstate.player = player;
                }
                4 => {
                    // way
                    commands
                        .spawn(SpriteBundle {
                            material: floor_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                FLOOR_LAYER,
                            )),
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);
                    commands
                        .spawn(SpriteBundle {
                            material: destructable_wall_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                OBJECT_LAYER,
                            )),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(Destructable::BombNumberBuffBox)
                        .with(InGame);
                }
                5 => {
                    // way
                    commands
                        .spawn(SpriteBundle {
                            material: floor_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                FLOOR_LAYER,
                            )),
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);
                    commands
                        .spawn(SpriteBundle {
                            material: destructable_wall_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                OBJECT_LAYER,
                            )),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(Destructable::PowerBuffBox)
                        .with(InGame);
                }
                6 => {
                    // way
                    commands
                        .spawn(SpriteBundle {
                            material: floor_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                FLOOR_LAYER,
                            )),
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);
                    commands
                        .spawn(SpriteBundle {
                            material: destructable_wall_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                OBJECT_LAYER,
                            )),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(Destructable::SpeedBuffBox)
                        .with(InGame);
                }
                _ => {
                    commands
                        .spawn(SpriteBundle {
                            material: floor_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                FLOOR_LAYER,
                            )),
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);
                }
            }
        }
    }
}
