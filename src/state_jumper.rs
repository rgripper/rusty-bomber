use crate::state::*;
use anyhow::Result;
use bevy::{app::AppExit, prelude::*};

pub fn jump_state(
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<State<GameState>>,
    input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) -> Result<()> {
    if app_state.current() != &AppState::StartMenu {
        if input.just_pressed(KeyCode::Back) {
            app_state.set_next(AppState::StartMenu)?;
            game_state.set_next(GameState::Invalid)?;
        }
    }
    if app_state.current() == &AppState::Game {
        if game_state.current() == &GameState::Game {
            if input.just_pressed(KeyCode::Escape) {
                game_state.set_next(GameState::Pause)?;
            }
        } else if game_state.current() == &GameState::Pause {
            if input.just_pressed(KeyCode::Escape) {
                game_state.set_next(GameState::Game)?;
            }
        } else if game_state.current() == &GameState::GameOver {
            if input.just_pressed(KeyCode::Return) {
                app_state.set_next(AppState::StartMenu)?;
                game_state.set_next(GameState::Invalid)?;
            }
            if input.just_pressed(KeyCode::Escape) {
                app_exit_events.send(AppExit);
            }
        }
    } else if app_state.current() == &AppState::StartMenu {
        if input.just_pressed(KeyCode::Return) {
            app_state.set_next(AppState::Game)?;
            game_state.set_next(GameState::Game)?;
        }
        if input.just_pressed(KeyCode::Escape) {
            app_exit_events.send(AppExit);
        }
    }
    //info!("app state:{:?} game state:{:?}",app_state.current(),game_state.current());
    return Ok(());
}
