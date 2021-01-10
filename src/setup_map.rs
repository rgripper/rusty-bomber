use crate::{
    assets::*,
    components::{Destructable, InGame, PlayerPosition, Wall, Way},
    constants::{FLOOR_LAYER, OBJECT_LAYER, PLAYER_LAYER},
    creatures::CreatureBundle,
    player::PlayerBundle,
    resources::Map,
    state::RunState,
    utils::{SCALE, TILE_WIDTH},
};
use bevy::prelude::*;

pub const GMAE_SETUP: &str = "game_setup";

pub fn setup_map(
    commands: &mut Commands,
    map_resource: Res<Map>,
    player_texture_atlas: Res<PlayerTextureAtlas>,
    floor_or_wall_texture_atlas: Res<FloorOrWallTextureAtlas>,
    creature_texture_atlas: Res<CreatureTextureAtlas>,
    mut runstate: ResMut<RunState>,
) {
    let room_map = map_resource.map_value();

    for (row_index, row) in room_map.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            // Using match here makes it easier to extend the map
            let wall_transform = Transform {
                translation: Vec3::new(
                    TILE_WIDTH * col_index as f32,
                    TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                    OBJECT_LAYER,
                ),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            };
            let way_transform = Transform {
                translation: Vec3::new(
                    TILE_WIDTH * col_index as f32,
                    TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                    FLOOR_LAYER,
                ),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            };

            match *cell {
                1 => {
                    commands
                        .spawn(SpriteSheetBundle {
                            transform: wall_transform,
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(5),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(InGame);
                }

                2 => {
                    // yellow way
                    commands
                        .spawn(SpriteSheetBundle {
                            transform: way_transform,
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(0),
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);
                    commands
                        .spawn(SpriteSheetBundle {
                            transform: wall_transform,
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(3),
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
                        .spawn(SpriteSheetBundle {
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(1),
                            transform: way_transform,
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);

                    // player
                    let player = commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: player_texture_atlas.0.clone(),
                            transform: Transform::from_scale(Vec3::splat(SCALE)),
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
                        .spawn(SpriteSheetBundle {
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(0),
                            transform: way_transform,
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);
                    commands
                        .spawn(SpriteSheetBundle {
                            transform: wall_transform,
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(3),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(Destructable::BombNumberBuffBox)
                        .with(InGame);
                }
                5 => {
                    // way
                    commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(0),
                            transform: way_transform,
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);
                    commands
                        .spawn(SpriteSheetBundle {
                            transform: wall_transform,
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(3),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(Destructable::PowerBuffBox)
                        .with(InGame);
                }
                6 => {
                    // way
                    commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(0),
                            transform: way_transform,
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);
                    commands
                        .spawn(SpriteSheetBundle {
                            transform: wall_transform,
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(3),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(Destructable::SpeedBuffBox)
                        .with(InGame);
                }
                7 => {
                    // way
                    commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(1),
                            transform: way_transform,
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);
                    // creature
                    let creature_transform = Transform {
                        translation: Vec3::new(
                            TILE_WIDTH * col_index as f32,
                            TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                            PLAYER_LAYER,
                        ),
                        scale: Vec3::splat(SCALE),
                        ..Default::default()
                    };
                    commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: creature_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(0),
                            transform: creature_transform,
                            ..Default::default()
                        })
                        .with_bundle(CreatureBundle::default())
                        .with(InGame)
                        .current_entity();
                }
                8 => {
                    // way
                    commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(0),
                            transform: way_transform,
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);
                    commands
                        .spawn(SpriteSheetBundle {
                            transform: wall_transform,
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(3),
                            ..Default::default()
                        })
                        .with(Destructable::Portal)
                        .with(Wall)
                        .with(InGame);
                }
                9 => {
                    commands
                        .spawn(SpriteSheetBundle {
                            transform: wall_transform,
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(4),
                            ..Default::default()
                        })
                        .with(Wall)
                        .with(InGame);
                }
                _ => {
                    commands
                        .spawn(SpriteSheetBundle {
                            texture_atlas: floor_or_wall_texture_atlas.0.clone(),
                            sprite: TextureAtlasSprite::new(1),
                            transform: way_transform,
                            ..Default::default()
                        })
                        .with(Way)
                        .with(InGame);
                }
            }
        }
    }
}
