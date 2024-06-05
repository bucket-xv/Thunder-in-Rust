//! This is the screen that is displayed when the game is won or lost.

use bevy::{app::AppExit, prelude::*};

use super::{despawn_screen, GameState, Level, TEXT_COLOR};

// This plugin manages the win/lose screen, with 3 different screens:
// - a win screen with "Main Menu", "Next Level", "Quit" buttons
// - a lose screen with "Main Menu", "Retry", "Quit" buttons
// - a completion screen with "Main Menu", "Quit" buttons
pub fn win_lose_screen_plugin(app: &mut App) {
    app.init_state::<WinLoseScreenState>()
        .add_systems(OnEnter(GameState::Win), win_screen_setup)
        .add_systems(OnEnter(GameState::Lose), lose_screen_setup)
        .add_systems(OnEnter(GameState::Completion), completion_screen_setup)
        .add_systems(
            OnEnter(WinLoseScreenState::BackToMainMenu),
            back_to_main_menu,
        )
        .add_systems(OnEnter(WinLoseScreenState::Restart), restart_level)
        .add_systems(OnEnter(WinLoseScreenState::NextLevel), next_level)
        .add_systems(
            Update,
            (win_lose_screen_action, button_system).run_if(in_state(WinLoseScreenState::WinScreen)),
        )
        .add_systems(
            Update,
            (win_lose_screen_action, button_system)
                .run_if(in_state(WinLoseScreenState::LoseScreen)),
        )
        .add_systems(
            Update,
            (win_lose_screen_action, button_system)
                .run_if(in_state(WinLoseScreenState::CompletionScreen)),
        )
        .add_systems(
            OnExit(WinLoseScreenState::WinScreen),
            despawn_screen::<OnWinScreen>,
        )
        .add_systems(
            OnExit(WinLoseScreenState::LoseScreen),
            despawn_screen::<OnLoseScreen>,
        )
        .add_systems(
            OnExit(WinLoseScreenState::CompletionScreen),
            despawn_screen::<OnCompleteScreen>,
        );
}

// State used for the win/lose screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum WinLoseScreenState {
    BackToMainMenu,
    Restart,
    NextLevel,
    #[default]
    Disabled,
}

#[derive(Component)]
struct OnCompleteScreen;

#[derive(Component)]
struct OnWinScreen;

#[derive(Component)]
struct OnLoseScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
enum WinLoseButtonAction {
    MainMenu,
    NextLevel,
    Retry,
    Quit,
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn win_screen_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnWinScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent: &mut ChildBuilder| {
                    // Display the title of the menu
                    parent.spawn(
                        TextBundle::from_section(
                            "You Win!",
                            TextStyle {
                                font_size: 60.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    // Display three buttons for each action available from the win screen:
                    // - Main Menu
                    // - Next Level
                    // - Retry
                    // - Quit
                    parent // Back to Main Menu
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            WinLoseButtonAction::MainMenu,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Main Menu",
                                button_text_style.clone(),
                            ));
                        });
                    parent // Retry
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            WinLoseButtonAction::Retry,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Retry Again",
                                button_text_style.clone(),
                            ));
                        });
                    parent // Next Level
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            WinLoseButtonAction::NextLevel,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Next Level",
                                button_text_style.clone(),
                            ));
                        });
                    parent // Quit
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            WinLoseButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent
                                .spawn(TextBundle::from_section("Quit", button_text_style.clone()));
                        });
                });
        });
}

fn lose_screen_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnWinScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent: &mut ChildBuilder| {
                    // Display the title of the menu
                    parent.spawn(
                        TextBundle::from_section(
                            "You Lose...",
                            TextStyle {
                                font_size: 60.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    // Display three buttons for each action available from the lose screen:
                    // - Main Menu
                    // - Retry
                    // - Quit
                    parent // Back to Main Menu
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            WinLoseButtonAction::MainMenu,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Main Menu",
                                button_text_style.clone(),
                            ));
                        });
                    parent // Retry
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            WinLoseButtonAction::Retry,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Retry Again",
                                button_text_style.clone(),
                            ));
                        });
                    parent // Quit
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            WinLoseButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent
                                .spawn(TextBundle::from_section("Quit", button_text_style.clone()));
                        });
                });
        });
}

fn completion_screen_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnWinScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent: &mut ChildBuilder| {
                    // Display the title of the menu
                    parent.spawn(
                        TextBundle::from_section(
                            "Congratulation! You passed all levels.",
                            TextStyle {
                                font_size: 50.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );

                    // Display three buttons for each action available from the complete screen:
                    // - Main Menu
                    // - Quit
                    parent // Back to Main Menu
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            WinLoseButtonAction::MainMenu,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Main Menu",
                                button_text_style.clone(),
                            ));
                        });
                    // parent // Retry
                    //     .spawn((
                    //         ButtonBundle {
                    //             style: button_style.clone(),
                    //             background_color: NORMAL_BUTTON.into(),
                    //             ..default()
                    //         },
                    //         WinLoseButtonAction::Retry,
                    //     ))
                    //     .with_children(|parent| {
                    //         let icon = asset_server.load("textures/Game Icons/right.png");
                    //         parent.spawn(ImageBundle {
                    //             style: button_icon_style.clone(),
                    //             image: UiImage::new(icon),
                    //             ..default()
                    //         });
                    //         parent.spawn(TextBundle::from_section(
                    //             "Retry Again",
                    //             button_text_style.clone(),
                    //         ));
                    //     });
                    parent // Quit
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            WinLoseButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent
                                .spawn(TextBundle::from_section("Quit", button_text_style.clone()));
                        });
                });
        });
}

fn win_lose_screen_action(
    interaction_query: Query<
        (&Interaction, &WinLoseButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut win_lose_screen_state: ResMut<NextState<WinLoseScreenState>>,
) {
    for (interaction, win_lose_buttton_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match win_lose_buttton_action {
                WinLoseButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                WinLoseButtonAction::MainMenu => {
                    win_lose_screen_state.set(WinLoseScreenState::BackToMainMenu);
                }
                WinLoseButtonAction::Retry => {
                    win_lose_screen_state.set(WinLoseScreenState::Restart);
                }
                WinLoseButtonAction::NextLevel => {
                    win_lose_screen_state.set(WinLoseScreenState::NextLevel);
                }
            }
        }
    }
}

fn back_to_main_menu(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Menu);
}

fn restart_level(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::LevelSplash);
}

fn next_level(mut level_setting: ResMut<Level>, mut state: ResMut<NextState<GameState>>) {
    *level_setting = Level(level_setting.0 + 1);
    state.set(GameState::LevelSplash);
}
