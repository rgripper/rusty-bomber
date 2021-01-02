use assets::*;
use bevy::prelude::*;
use errors::error_handler;
use events::{GameOverEvent, RecoveryBombNumberEvent};
use resources::Map;
use state::*;
use state_jumper::jump_state;
use ui::{draw_blink_system, ButtonMaterials};

use creatures::*;

pub mod assets;

pub mod animate;
pub mod bomb;
pub mod buff;
pub mod bundle;
pub mod components;
pub mod constants;
pub mod creatures;
pub mod errors;
pub mod events;
pub mod movement;
pub mod resources;
pub mod setup_map;
pub mod state;
pub mod state_jumper;
pub mod ui;
pub mod utils;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(Map::first())
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
    let player_texture_handle = asset_server.load("chars/sample_character_01.png");
    let player_texture_atlas =
        TextureAtlas::from_grid(player_texture_handle, Vec2::new(16.0, 32.0), 4, 3);
    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        .insert_resource(PermaWallMaterial(
            materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        ))
        .insert_resource(DestructableWallMaterial(
            materials.add(Color::rgb(1.0, 1.0, 0.7).into()),
        ))
        .insert_resource(FloorMaterial(
            materials.add(Color::rgb(0.5, 1.0, 0.5).into()),
        ))
        .insert_resource(PlayerTextureAtlas(player_texture_atlas_handle))
        .insert_resource(CreatureMaterial(
            materials.add(Color::rgb(1.0, 0.3, 0.5).into()),
        ))
        .insert_resource(BombMaterial(
            materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
        ))
        .insert_resource(FireMaterial(
            materials.add(Color::rgb(1.0, 0.2, 0.2).into()),
        ))
        .insert_resource(PowerBuffMaterial(
            materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
        ))
        .insert_resource(SpeedBuffMaterial(
            materials.add(Color::rgb(0.0, 1.0, 1.0).into()),
        ))
        .insert_resource(BombNumberBuffMaterial(
            materials.add(Color::rgb(1.0, 1.0, 0.0).into()),
        ))
        .insert_resource(LifeMaterial(
            materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        ));
    commands.insert_resource(RunState::new(&asset_server));
}
