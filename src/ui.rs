use bevy::prelude::*;

use crate::{
    components::InGame,
    constants::START_SPEED,
    resources::Map,
    state::{AppState, GameState, RunState},
};

pub struct DrawBlinkTimer(pub Timer);
pub struct WillDestroy;

pub fn start_menu(
    commands: &mut Commands,
    runstate: Res<RunState>,
    button_materials: Res<ButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with(WillDestroy)
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        value: "Bomberman".to_string(),
                        font: runstate.font_handle.clone(),
                        style: TextStyle {
                            font_size: 100.0,
                            color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(8.0)),
                        // center button
                        margin: Rect::all(Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle {
                            text: Text {
                                value: "enter".to_string(),
                                font: runstate.font_handle.clone(),
                                style: TextStyle {
                                    font_size: 50.0,
                                    color: Color::rgb_u8(0x00, 0x44, 0x44),
                                    ..Default::default()
                                },
                            },
                            ..Default::default()
                        })
                        .with(DrawBlinkTimer(Timer::from_seconds(0.5, true)));
                });
        });
}
pub struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgba(0.15, 0.15, 0.15, 0.0).into()),
            hovered: materials.add(Color::rgba(0.25, 0.25, 0.25, 0.51).into()),
            pressed: materials.add(Color::rgba(0.35, 0.35, 0.35, 0.21).into()),
        }
    }
}

pub fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut app_state: ResMut<State<AppState>>,
    mut game_state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Mutated<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                app_state.set_next(AppState::Game).unwrap();
                game_state.set_next(GameState::Game).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

pub fn draw_blink_system(time: Res<Time>, mut query: Query<(&mut DrawBlinkTimer, &mut Visible)>) {
    for (mut timer, mut visible) in query.iter_mut() {
        timer.0.tick(time.delta_seconds());
        if timer.0.finished() {
            visible.is_visible = !visible.is_visible;
        }
    }
}

pub struct SpeedUi;
pub struct BumbPowerUi;
pub struct RremainBumbNumUi;

pub fn spawn_game_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    runstate: Res<RunState>,
) {
    if let Some(_player) = runstate.player {
        commands
            // speed value
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexEnd,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with(WillDestroy)
            .with_children(|parent| {
                parent
                    .spawn(TextBundle {
                        style: Style {
                            justify_content: JustifyContent::FlexEnd,
                            margin: Rect {
                                left: Val::Px(10.0),
                                right: Val::Px(10.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(10.0),
                            },
                            ..Default::default()
                        },
                        text: Text {
                            value: START_SPEED.to_string(),
                            font: runstate.font_handle.clone(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    })
                    .with(SpeedUi);
            })
            // Bumb number counters
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with(WillDestroy)
            .with_children(|parent| {
                parent
                    .spawn(ImageBundle {
                        style: Style {
                            margin: Rect {
                                left: Val::Px(10.0),
                                right: Val::Px(10.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(10.0),
                            },
                            ..Default::default()
                        },
                        material: materials.add(asset_server.load("Bomb_1.png").into()),
                        ..Default::default()
                    })
                    .with(RremainBumbNumUi);
            })
            // bumb power level
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexEnd,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with(WillDestroy)
            .with_children(|parent| {
                parent
                    .spawn(TextBundle {
                        style: Style {
                            justify_content: JustifyContent::FlexEnd,
                            margin: Rect {
                                left: Val::Px(10.0),
                                right: Val::Px(10.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(10.0),
                            },
                            ..Default::default()
                        },
                        text: Text {
                            value: "1".to_string(),
                            font: runstate.font_handle.clone(),
                            style: TextStyle {
                                font_size: 50.0,
                                color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    })
                    .with(BumbPowerUi);
            });
    }
}

//pub fn setting_ui(commands: &mut Commands, runstate: ResMut<RunState>) {}

pub fn pause_menu(
    commands: &mut Commands,
    runstate: ResMut<RunState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with(WillDestroy)
        .with(InGame)
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        value: "pause".to_string(),
                        font: runstate.font_handle.clone(),
                        style: TextStyle {
                            font_size: 100.0,
                            color: Color::rgb_u8(0xF8, 0xE4, 0x73),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(DrawBlinkTimer(Timer::from_seconds(0.5, true)));
        });
}

pub fn gameover_menu(
    commands: &mut Commands,
    runstate: ResMut<RunState>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with(WillDestroy)
        .with(InGame)
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        value: "Game Over".to_string(),
                        font: runstate.font_handle.clone(),
                        style: TextStyle {
                            font_size: 100.0,
                            color: Color::rgb_u8(0xAA, 0x22, 0x22),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        value: "enter".to_string(),
                        font: runstate.font_handle.clone(),
                        style: TextStyle {
                            font_size: 50.0,
                            color: Color::rgb_u8(0x88, 0x22, 0x22),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(DrawBlinkTimer(Timer::from_seconds(0.5, true)));
        });
}
pub fn game_victory(
    commands: &mut Commands,
    runstate: ResMut<RunState>,
    map: Res<Map>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with(WillDestroy)
        .with(InGame)
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        value: "Victory".to_string(),
                        font: runstate.font_handle.clone(),
                        style: TextStyle {
                            font_size: 100.0,
                            color: Color::rgb_u8(0xAA, 0x22, 0x22),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .spawn(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text: Text {
                        value: if !map.is_final {
                            "next level".to_string()
                        } else {
                            "All clear!Enter to random level.".to_string()
                        },
                        font: runstate.font_handle.clone(),
                        style: TextStyle {
                            font_size: 50.0,
                            color: Color::rgb_u8(0x88, 0x22, 0x22),
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                })
                .with(DrawBlinkTimer(Timer::from_seconds(0.5, true)));
        });
}
