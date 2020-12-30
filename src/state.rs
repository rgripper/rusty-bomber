use bevy::prelude::*;

use crate::{animate::AnimateSystems, bomb::BombSystems, buff::BuffSystems, components::InGame, events::GameOverEvent, movement::MovementSystems, setup_map::setup_map, ui::{button_system, gameover_menu, pause_menu, start_menu, WillDestroy}};

#[derive(Clone, PartialEq)]
pub enum AppState {
    StartMenu,
    Game,
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
                            .movement_systems()
                            .bomb_systems()
                            .buff_systems()
                            .animate_systems()
                            .add_system(game_over_events.system())
                    })
                    .on_state_exit(AppState::Game, exit_game_despawn.system())
            });
    }
}
fn exit_ui_despawn(commands: &mut Commands, query: Query<Entity, With<WillDestroy>>) {
    for entity in query.iter() {
        commands.despawn_recursive(entity);
    }
}
fn exit_game_despawn(commands: &mut Commands, query: Query<Entity, With<InGame>>) {
    for entity in query.iter() {
        commands.despawn_recursive(entity);
    }
}
#[derive(Clone, PartialEq)]
pub enum GameState {
    Invalid,
    Game,
    Pause,
    GameOver,
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
                    .on_state_exit(GameState::Pause, exit_ui_despawn.system())
                    .on_state_enter(GameState::GameOver, gameover_menu.system())
                    .on_state_exit(GameState::GameOver, exit_ui_despawn.system())
            });
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

fn game_over_events(
    events: Res<Events<GameOverEvent>>,
    mut reader: Local<EventReader<GameOverEvent>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for _ in reader.iter(&events) {
        match game_state.set_next(GameState::GameOver) {
            Ok(_) => {
                info!("Game Over!");
                break;
            }
            Err(err) => error!("{}", err),
        }
    }
}
