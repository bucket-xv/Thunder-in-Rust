//! This is the page that appears after the level is selected and before the corresponding game scene will
//! start, which will display the selected level to the user again.
//! It will display the level for 1 second before transitioning to the main menu.
use bevy::prelude::*;

use super::{despawn_screen, GameState, Level, TEXT_COLOR};

// This plugin will display a splash screen with the level information for 1 second before switching to the menu
pub fn level_splash_plugin(app: &mut App) {
    // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
    app
        // When entering the state, spawn everything needed for this screen
        .add_systems(OnEnter(GameState::LevelSplash), level_splash_setup)
        // While in this state, run the `countdown` system
        .add_systems(Update, countdown.run_if(in_state(GameState::LevelSplash)))
        // When exiting the state, despawn everything that was spawned for this screen
        .add_systems(
            OnExit(GameState::Splash),
            despawn_screen::<OnLevelSplashScreen>,
        );
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnLevelSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn level_splash_setup(mut commands: Commands, level: Res<Level>) {
    // Display the level information
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnLevelSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    format!("Level {}", level.0),
                    TextStyle {
                        font_size: 100.0,
                        color: TEXT_COLOR,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );
        });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Game);
    }
}
