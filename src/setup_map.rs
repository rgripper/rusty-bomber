use crate::{
    assets::*,
    components::{AnimateIndexs, Player},
    entities::{
        create_bomb_number_buff_box, create_creature, create_green_way, create_last_wall,
        create_normal_box, create_normal_wall, create_player, create_portal_box,
        create_power_buff_box, create_speed_buff_box,
    },
    resources::Map,
    state::RunState,
    utils::index_to_position,
};
use bevy::prelude::*;

pub fn setup_map(
    commands: &mut Commands,
    room_map: Res<Map>,
    player_texture_atlas: Res<PlayerTextureAtlas>,
    floor_or_wall_texture_atlas: Res<FloorOrWallTextureAtlas>,
    player_animate_indexs: Res<AnimateIndexs<Player>>,
    //creature_texture_atlas: Res<CreatureTextureAtlas>,
    mut runstate: ResMut<RunState>,
) {
    for (row_index, row) in room_map.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            // Using match here makes it easier to extend the map
            let translation = index_to_position(col_index, row_index, room_map.len());

            match *cell {
                1 => {
                    create_normal_wall(commands, translation, floor_or_wall_texture_atlas.0.clone())
                }

                2 => {
                    create_normal_box(commands, translation, floor_or_wall_texture_atlas.0.clone())
                }
                // When setting each level, the player’s position should be set flexibly
                3 => {
                    // player
                    let player = create_player(
                        commands,
                        translation,
                        floor_or_wall_texture_atlas.0.clone(),
                        player_texture_atlas.0.clone(),
                        player_animate_indexs.up[0],
                    );
                    runstate.player = player;
                }
                4 => {
                    create_bomb_number_buff_box(
                        commands,
                        translation,
                        floor_or_wall_texture_atlas.0.clone(),
                    );
                }
                5 => {
                    create_power_buff_box(
                        commands,
                        translation,
                        floor_or_wall_texture_atlas.0.clone(),
                    );
                }
                6 => {
                    // way
                    create_speed_buff_box(
                        commands,
                        translation,
                        floor_or_wall_texture_atlas.0.clone(),
                    );
                }
                7 => {
                    create_creature(
                        commands,
                        translation,
                        floor_or_wall_texture_atlas.0.clone(),
                        player_texture_atlas.0.clone(), //TODO:creature_texture_atlas
                    );
                }
                8 => {
                    create_portal_box(commands, translation, floor_or_wall_texture_atlas.0.clone());
                }
                9 => {
                    create_last_wall(commands, translation, floor_or_wall_texture_atlas.0.clone());
                }
                _ => {
                    create_green_way(commands, translation, floor_or_wall_texture_atlas.0.clone());
                }
            }
        }
    }
}
