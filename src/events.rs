use crate::{
    components::{BombNumber, Player},
    resources::Map,
    state::*,
};
use anyhow::Result;
use bevy::{app::AppExit, prelude::*};
use bevy_rapier2d::physics::RapierConfiguration;
pub enum GameEvents {
    GameOver,
    Victory,
    RecoveryBombNumber(Entity),
}
pub fn game_events_handle(
    game_events: Res<Events<GameEvents>>,
    mut events_reader: Local<EventReader<GameEvents>>,
    mut player_query: Query<(Entity, &mut BombNumber), With<Player>>,
    mut game_state: ResMut<State<GameState>>,
    mut physics_state: ResMut<RapierConfiguration>,
) -> Result<()> {
    for event in events_reader.iter(&game_events) {
        match event {
            GameEvents::GameOver => match game_state.current() {
                GameState::GameOver | GameState::Victory | GameState::Invalid => {}
                _ => {
                    physics_state.physics_pipeline_active = false;
                    game_state.set_next(GameState::GameOver)?
                }
            },
            GameEvents::Victory => match game_state.current() {
                GameState::GameOver | GameState::Victory | GameState::Invalid => {}
                _ => {
                    physics_state.physics_pipeline_active = false;
                    game_state.set_next(GameState::Victory)?
                }
            },
            GameEvents::RecoveryBombNumber(entity) => {
                'bomb_number: for (player, mut number) in player_query.iter_mut() {
                    if entity == &player {
                        number.current -= 1;
                        break 'bomb_number;
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn jump_state(
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<State<GameState>>,
    mut map: ResMut<Map>,
    input: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) -> Result<()> {
    if app_state.current() != &AppState::StartMenu {
        if input.just_pressed(KeyCode::Back) {
            app_state.set_next(AppState::StartMenu)?;
            game_state.set_next(GameState::Invalid)?;
            map.init();
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
                map.init();
            }
            if input.just_pressed(KeyCode::Escape) {
                app_exit_events.send(AppExit);
            }
        } else if game_state.current() == &GameState::Victory {
            if input.just_pressed(KeyCode::Return) {
                map.next();
                app_state.set_next(AppState::Temporary)?;
                game_state.set_next(GameState::Game)?;
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
    Ok(())
}
