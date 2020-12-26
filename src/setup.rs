use crate::Direction;
use crate::*;

pub const GMAE_SETUP: &str = "game_setup";

pub fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(PermaWallMaterial(
            materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        ))
        .insert_resource(DestructableWallMaterial(
            materials.add(Color::rgb(1.0, 1.0, 0.7).into()),
        ))
        .insert_resource(FloorMaterial(
            materials.add(Color::rgb(0.5, 1.0, 0.5).into()),
        ))
        .insert_resource(PlayerMaterial(
            materials.add(Color::rgb(0.7, 0.5, 1.0).into()),
        ))
        .insert_resource(CreatureMaterial(
            materials.add(Color::rgb(1.0, 0.3, 0.5).into()),
        ))
        .insert_resource(BombMaterial(
            materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
        ))
        .insert_resource(FireMaterial(
            materials.add(Color::rgb(1.0, 0.2, 0.2).into()),
        ));
}

pub fn game_setup_room(
    commands: &mut Commands,
    perma_wall_material: Res<PermaWallMaterial>,
    map_resource: Res<Map>,
    destructable_wall_material: Res<DestructableWallMaterial>,
    player_material: Res<PlayerMaterial>,
    floor_material: Res<FloorMaterial>,
    //mut wall_position: Query<(&Wall, &mut Transform)>,
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
                        .with(Wall);
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
                        .with(Way);
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
                        .with(Destructable);
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
                        .with(Way);

                    // player
                    commands
                        .spawn(SpriteBundle {
                            material: player_material.0.clone(),
                            sprite: Sprite::new(Vec2::new(TILE_WIDTH as f32, TILE_WIDTH as f32)),
                            transform: Transform::from_translation(Vec3::new(
                                TILE_WIDTH * col_index as f32,
                                TILE_WIDTH * (room_map.len() - row_index - 1) as f32,
                                PLAYER_LAYER,
                            )),
                            ..Default::default()
                        })
                        .with(Player { is_moving: false })
                        .with(Direction::Right)
                        .with(Velocity(1.0))
                        .with(Destructable);
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
                        .with(Way);
                }
            }
        }
    }

    // for (_player, mut transform) in &mut player_position.iter_mut() {
    //     transform.translation += Vec3::new(10.0, 0.0, 0.0);
    // }
}
