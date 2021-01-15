use assets::*;
use bevy::prelude::*;
use errors::error_handler;
use events::{GameOverEvent, RecoveryBombNumberEvent};
use resources::{Map, MAX_HEIGHT, MAX_WIDTH};
use state::*;
use state_jumper::jump_state;
use ui::{draw_blink_system, ButtonMaterials};
use utils::TILE_WIDTH;

pub mod assets;
pub mod bomb;
pub mod buff;
pub mod components;
pub mod constants;
pub mod creatures;
pub mod entitys;
pub mod errors;
pub mod events;
pub mod player;
pub mod portal;
pub mod resources;
pub mod setup_map;
pub mod state;
pub mod state_jumper;
pub mod ui;
pub mod utils;

fn main() {
    let mut app = App::build();
    app.add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "BomberMan".to_string(),
            width: MAX_WIDTH,
            height: MAX_HEIGHT,
            resizable: true,
            // mode: window::WindowMode::Fullscreen {use_size: false},
            mode: bevy::window::WindowMode::Windowed,
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#bevy-canvas".to_string()),
            //vsync: true,
            ..Default::default()
        });
    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(DefaultPlugins);
    #[cfg(target_arch = "wasm32")]
    app.add_plugins(bevy_webgl2::DefaultPlugins);
    app.add_resource(Map::first())
        .init_resource::<ButtonMaterials>()
        .add_event::<RecoveryBombNumberEvent>()
        .add_event::<GameOverEvent>()
        .add_plugin(AppStatePluge)
        .add_plugin(GameStatePlugin)
        .add_startup_system(setup.system())
        .add_system(draw_blink_system.system())
        .add_system(jump_state.system().chain(error_handler.system()))
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player_texture_handle = asset_server.load("player.png");
    let player_texture_atlas =
        TextureAtlas::from_grid(player_texture_handle, Vec2::new(16.0, 16.0), 14, 4);
    let bomb_texture_handle = asset_server.load("bomb.png");
    let bomb_texture_atlas =
        TextureAtlas::from_grid(bomb_texture_handle, Vec2::new(16.0, 16.0), 3, 1);
    let fire_texture_handle = asset_server.load("fire.png");
    let fire_texture_atlas =
        TextureAtlas::from_grid(fire_texture_handle, Vec2::new(16.0, 16.0), 4, 3);
    let floor_or_wall_texture_handle = asset_server.load("wall.png");
    let floor_or_wall_texture_atlas =
        TextureAtlas::from_grid(floor_or_wall_texture_handle, Vec2::new(16.0, 16.0), 6, 1);
    let creature_texture_handle = asset_server.load("creature.png");
    let creature_texture_atlas =
        TextureAtlas::from_grid(creature_texture_handle, Vec2::new(16.0, 16.0), 14, 1);
    let portal_texture_handle = asset_server.load("door.png");
    let portal_texture_atlas =
        TextureAtlas::from_grid(portal_texture_handle, Vec2::new(16.0, 16.0), 2, 1);

    commands
        // cameras
        .spawn(Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(
                    (MAX_WIDTH - TILE_WIDTH) / 2.0,
                    (MAX_HEIGHT - TILE_WIDTH) / 2.0,
                    20.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .spawn(CameraUiBundle::default())
        .insert_resource(PlayerTextureAtlas(
            texture_atlases.add(player_texture_atlas),
        ))
        .insert_resource(BombTextureAtlas(texture_atlases.add(bomb_texture_atlas)))
        .insert_resource(FireTextureAtlas(texture_atlases.add(fire_texture_atlas)))
        .insert_resource(PortalTextureAtlas(
            texture_atlases.add(portal_texture_atlas),
        ))
        .insert_resource(FloorOrWallTextureAtlas(
            texture_atlases.add(floor_or_wall_texture_atlas),
        ))
        .insert_resource(CreatureTextureAtlas(
            texture_atlases.add(creature_texture_atlas),
        ))
        .insert_resource(PowerBuffMaterial(
            materials.add(asset_server.load("power_icon.png").into()),
        ))
        .insert_resource(SpeedBuffMaterial(
            materials.add(asset_server.load("speed_icon.png").into()),
        ))
        .insert_resource(BombNumberBuffMaterial(
            materials.add(asset_server.load("bomb_icon.png").into()),
        ))
        .insert_resource(LifeMaterial(
            materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        ));
    commands.insert_resource(RunState::new(&asset_server));
}
