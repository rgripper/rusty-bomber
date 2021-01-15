use crate::{
    assets::*,
    components::{Destructible, InGame, Wall, Way},
    constants::{FLOOR_LAYER, OBJECT_LAYER, PLAYER_LAYER},
    creatures::CreatureBundle,
    player::PlayerBundle,
    resources::Map,
    state::RunState,
    utils::{index_to_position, SCALE},
};
use bevy::prelude::*;

pub const GMAE_SETUP: &str = "game_setup";

pub fn setup_map(
    commands: &mut Commands,
    room_map: Res<Map>,
    player_texture_atlas: Res<PlayerTextureAtlas>,
    floor_or_wall_texture_atlas: Res<FloorOrWallTextureAtlas>,
    creature_texture_atlas: Res<CreatureTextureAtlas>,
    mut runstate: ResMut<RunState>,
) {
    for (row_index, row) in room_map.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            // Using match here makes it easier to extend the map
            let wall_transform = Transform {
                translation: index_to_position(col_index, row_index, room_map.len(), OBJECT_LAYER),
                scale: Vec3::splat(SCALE),
                ..Default::default()
            };
            let way_transform = Transform {
                translation: index_to_position(col_index, row_index, room_map.len(), FLOOR_LAYER),
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
                        .with(Destructible::NormalBox)
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
                            transform: Transform {
                                translation: index_to_position(
                                    col_index,
                                    row_index,
                                    room_map.len(),
                                    PLAYER_LAYER,
                                ),

                                scale: Vec3::splat(SCALE),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_bundle(PlayerBundle::default())
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
                        .with(Destructible::BombNumberBuffBox)
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
                        .with(Destructible::PowerBuffBox)
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
                        .with(Destructible::SpeedBuffBox)
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
                        translation: index_to_position(
                            col_index,
                            row_index,
                            room_map.len(),
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
                        .with(Destructible::Portal)
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
