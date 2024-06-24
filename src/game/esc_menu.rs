//! This is the menu that appears when the player presses the escape key
//! or the player press the "menu" button during the game.

use bevy::{app::AppExit, prelude::*};

use super::{despawn_screen, GameState};

use crate::menu::HELP_SCREEN;

// This plugin manages the escape menu, with 2 different screens:
// - a main menu with "Main Menu", "Back to Game" and "Quit" buttons
pub fn esc_menu_plugin(app: &mut App) {
    app.init_state::<EscMenuState>()
        .add_systems(OnEnter(GameState::Stopped), esc_menu_setup)
        .add_systems(OnEnter(EscMenuState::MainEscMenu), esc_main_menu_setup)
        .add_systems(OnEnter(EscMenuState::BackToMainMenu), back_to_main_menu)
        .add_systems(OnEnter(EscMenuState::BackToGame), back_to_game)
        .add_systems(OnEnter(EscMenuState::Help), help_screen_setup)
        .add_systems(OnExit(EscMenuState::Help), despawn_screen::<OnHelpScreen>)
        .add_systems(
            Update,
            (esc_menu_action, button_system).run_if(in_state(EscMenuState::Help)),
        )
        .add_systems(
            Update,
            (esc_menu_action, button_system).run_if(in_state(EscMenuState::MainEscMenu)),
        )
        .add_systems(
            OnExit(EscMenuState::MainEscMenu),
            despawn_screen::<OnMainEscMenuScreen>,
        );
}

// Statte used for the escape menu
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum EscMenuState {
    MainEscMenu, // The main menu screen
    BackToGame,  // The screen that appears when the player clicks the "Back to Game" button
    BackToMainMenu,
    Help,
    #[default]
    Disabled,
}

#[derive(Component)]
struct OnMainEscMenuScreen;

#[derive(Component)]
struct OnHelpScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
enum EscMenuButtonAction {
    BackToEscMenu,
    BackToGame,
    BackToMainMenu,
    Quit,
    GoToHelp,
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

fn esc_menu_setup(mut esc_menu_state: ResMut<NextState<EscMenuState>>) {
    esc_menu_state.set(EscMenuState::MainEscMenu);
}

fn esc_main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            OnMainEscMenuScreen,
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
                            "GAME PAUSE",
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

                    // Display three buttons for each action available from the esc main menu:
                    // - Home
                    // - Continue
                    // - Help
                    // - Exit
                    parent // Home
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            EscMenuButtonAction::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/home.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent
                                .spawn(TextBundle::from_section("Home", button_text_style.clone()));
                        });
                    parent // Continue
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            EscMenuButtonAction::BackToGame,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Continue",
                                button_text_style.clone(),
                            ));
                        });
                    parent // Continue
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            EscMenuButtonAction::GoToHelp,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/help.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent
                                .spawn(TextBundle::from_section("Help", button_text_style.clone()));
                        });
                    parent // Exit
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            EscMenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("textures/Game Icons/exitRight.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..default()
                            });
                            parent
                                .spawn(TextBundle::from_section("Exit", button_text_style.clone()));
                        });
                });
        });
}

fn esc_menu_action(
    interaction_query: Query<
        (&Interaction, &EscMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut esc_menu_state: ResMut<NextState<EscMenuState>>,
    // mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, esc_menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match esc_menu_button_action {
                EscMenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
                EscMenuButtonAction::BackToMainMenu => {
                    esc_menu_state.set(EscMenuState::BackToMainMenu);
                }
                EscMenuButtonAction::BackToGame => {
                    esc_menu_state.set(EscMenuState::BackToGame);
                }
                EscMenuButtonAction::GoToHelp => {
                    esc_menu_state.set(EscMenuState::Help);
                }
                EscMenuButtonAction::BackToEscMenu => {
                    esc_menu_state.set(EscMenuState::MainEscMenu);
                }
            }
        }
    }
}

fn back_to_main_menu(
    mut state: ResMut<NextState<GameState>>,
    mut esc_menu_state: ResMut<NextState<EscMenuState>>,
) {
    state.set(GameState::Menu);
    esc_menu_state.set(EscMenuState::Disabled);
}

fn back_to_game(
    mut state: ResMut<NextState<GameState>>,
    mut esc_menu_state: ResMut<NextState<EscMenuState>>,
) {
    state.set(GameState::Game);
    esc_menu_state.set(EscMenuState::Disabled);
}

fn help_screen_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
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
            OnHelpScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "HELP",
                            TextStyle {
                                font_size: 60.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::new(
                                Val::Px(50.0),
                                Val::Px(50.0),
                                Val::Px(50.0),
                                Val::Px(10.0),
                            ),
                            ..default()
                        }),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            HELP_SCREEN,
                            TextStyle {
                                font_size: 25.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            padding: UiRect::all(Val::Px(20.0)),
                            margin: UiRect::all(Val::Px(50.0)),
                            flex_wrap: FlexWrap::Wrap,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        }),
                    );
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            EscMenuButtonAction::BackToEscMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Back", button_text_style));
                        });
                });
        });
}
