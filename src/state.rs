use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;

use crate::{
    bomb::BombSystems,
    buff::BuffSystems,
    components::{InGame, Player, Stop},
    creatures::{Creature, CreatureSystems},
    player::PlayerSystems,
    portal::PortalSystems,
    setup_map::setup_map,
    ui::{button_system, game_victory, gameover_menu, pause_menu, start_menu, WillDestroy},
};

#[derive(Clone, PartialEq, Debug)]
pub enum AppState {
    StartMenu,
    Game,
    Temporary,
}
const APP_STATE_STAGE: &str = "app_state";
pub struct AppStatePluge;

impl Plugin for AppStatePluge {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(State::new(AppState::StartMenu))
            .add_stage_after(
                stage::UPDATE,
                APP_STATE_STAGE,
                StateStage::<AppState>::default(),
            )
            .stage(APP_STATE_STAGE, |stage: &mut StateStage<AppState>| {
                stage
                    // start menu
                    .on_state_enter(AppState::StartMenu, start_menu.system())
                    .on_state_update(AppState::StartMenu, button_system.system())
                    .on_state_exit(AppState::StartMenu, exit_ui_despawn.system())
                    // in game
                    .on_state_enter(AppState::Game, setup_map.system())
                    //.on_state_enter(AppState::Game, spawn_game_ui.system())
                    .update_stage(AppState::Game, |stage: &mut SystemStage| {
                        stage
                            .player_systems()
                            .bomb_systems()
                            .buff_systems()
                            .creature_systems()
                            .portal_systems()
                    })
                    .on_state_exit(AppState::Game, exit_game_despawn.system())
                    .on_state_enter(AppState::Temporary, jump_game.system())
            });
    }
}
fn jump_game(mut app_state: ResMut<State<AppState>>) {
    app_state.set_next(AppState::Game).unwrap();
}
fn exit_ui_despawn(commands: &mut Commands, query: Query<Entity, With<WillDestroy>>) {
    for entity in query.iter() {
        commands.despawn_recursive(entity);
    }
}
fn exit_game_ui_despawn(
    commands: &mut Commands,
    query: Query<Entity, (With<WillDestroy>, With<InGame>)>,
    mut configuration: ResMut<RapierConfiguration>,
) {
    configuration.physics_pipeline_active = true;
    for entity in query.iter() {
        commands.despawn_recursive(entity);
    }
}
fn exit_game_despawn(commands: &mut Commands, query: Query<Entity, With<InGame>>) {
    for entity in query.iter() {
        commands.despawn_recursive(entity);
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum GameState {
    Invalid,
    Game,
    Pause,
    GameOver,
    Victory,
}
const GAME_STATE_STAGE: &str = "game_state";
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(State::new(GameState::Invalid))
            .add_stage_after(
                APP_STATE_STAGE,
                GAME_STATE_STAGE,
                StateStage::<GameState>::default(),
            )
            .stage(GAME_STATE_STAGE, |stage: &mut StateStage<GameState>| {
                stage
                    .on_state_enter(GameState::Pause, pause_menu.system())
                    .on_state_enter(GameState::Pause, pause_enter.system())
                    .on_state_exit(GameState::Pause, exit_game_ui_despawn.system())
                    .on_state_exit(GameState::Pause, pause_exit.system())
                    .on_state_enter(GameState::GameOver, gameover_menu.system())
                    .on_state_exit(GameState::GameOver, exit_game_ui_despawn.system())
                    .on_state_enter(GameState::Victory, game_victory.system())
                    .on_state_exit(GameState::Victory, exit_game_ui_despawn.system())
            });
    }
}
fn pause_enter(
    commands: &mut Commands,
    player_query: Query<Entity, (With<Player>, Without<Stop>)>,
    creature_query: Query<Entity, (With<Creature>, Without<Stop>)>,
) {
    for entity in player_query.iter() {
        commands.insert_one(entity, Stop);
    }
    for entity in creature_query.iter() {
        commands.insert_one(entity, Stop);
    }
}
fn pause_exit(commands: &mut Commands, query: Query<Entity, With<Stop>>) {
    for entity in query.iter() {
        commands.remove_one::<Stop>(entity);
    }
}
pub struct RunState {
    pub player: Option<Entity>,
    pub font_handle: Handle<Font>,
    pub level: Option<i32>,
}

impl RunState {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            player: None,
            font_handle: asset_server.load("fonts/FiraMono-Medium.ttf"),
            level: None,
        }
    }
}
