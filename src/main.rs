//! This example will display a simple menu using Bevy UI where you can start a new game,
//! change some settings or quit. There is no actual game, it will just display the current
//! settings for 5 seconds before going back to the menu.

mod game;
mod menu;
mod splash;

use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
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
enum Level {
    Unset,
    Level1,
    Level2, 
    Level3, 
    Level4, 
    Level5, 
    Level6,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .insert_resource(Level::Unset)
        // .insert_resource(ResolutionSettings {
        //     large: Vec2::new(1920.0, 1080.0),
        //     medium: Vec2::new(800.0, 600.0),
        //     small: Vec2::new(640.0, 360.0),
        // })
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
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
