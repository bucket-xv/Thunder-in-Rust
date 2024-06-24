//! This is the main file that controls the general settings of the game.

mod animes;
mod game;
mod level_splash;
mod menu;
mod splash;

use bevy::prelude::*;
// use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;
use bevy_rand::prelude::WyRand;
use bevy_spritesheet_animation::prelude::*;
use animes::animate_sprite;
// use rand_core::RngCore;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    LevelSplash,
    Game,
    Stopped,
    Win,
    Lose,
    Completion,
    Init,
}

// Display settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// Volume settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

// Levels to play that can be choose in the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Level(u32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // *** begin bevy_spritesheet_animation example comment
        // Add the plugin to enable animations.
        // This makes the SpritesheetLibrary resource available to your systems.
        // *** end bevy_spritesheet_animation example comment
        .add_plugins(SpritesheetAnimationPlugin)
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .insert_resource(Level(0))
        // .insert_resource(ResolutionSettings {
        //     large: Vec2::new(1920.0, 1080.0),
        //     medium: Vec2::new(800.0, 600.0),
        //     small: Vec2::new(640.0, 360.0),
        // })
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        // for test
        //.add_systems(Startup, animes::setup_character)
        // Adds the plugins for each state
        .add_plugins((
            splash::splash_plugin,
            menu::menu_plugin,
            game::esc_menu::esc_menu_plugin,
            level_splash::level_splash_plugin,
            game::game_plugin,
            game::win_lose_screen::win_lose_screen_plugin,
            // This is a random generator plugin
            EntropyPlugin::<WyRand>::default(),
        ))
        .run();
}

// /// Stores the various window-resolutions we can select between.
// #[derive(Resource)]
// struct ResolutionSettings {
//     large: Vec2,
//     medium: Vec2,
//     small: Vec2,
// }

fn setup(
    mut commands: Commands,
    // mut windows: Query<&mut Window>,
    // resolution: Res<ResolutionSettings>,
) {
    commands.spawn(Camera2dBundle::default());
    // let mut window = windows.single_mut();
    // let res = resolution.medium;
    // window.resolution.set(res.x, res.y);
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
