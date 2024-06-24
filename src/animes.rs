//! This is the anime implementations of Thunder.

// TODO | Insert the animes into game.
// TODO | Write asset files.

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        // info!("animating sprite, index = {}", atlas.index);
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

/// private void setup_anime_periodical(...){...}
/// Set up periodical animes with the given resource file in the given directory.
fn setup_anime_periodical(
    mut commands: Commands,
    mut library: ResMut<SpritesheetLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
    texture_dir: String,
    number_of_frames: usize,
) {
    // Create an animation

    // println!("texture_dir: {}", texture_dir);

    let clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(number_of_frames, 1).row(0));
    });

    let animation_id = library.new_animation(|animation| {
        animation.add_stage(clip_id.into());
    });

    // Spawn a sprite using Bevy's built-in SpriteSheetBundle

    let texture = assets.load(texture_dir);

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(96.0, 96.0),
        number_of_frames,
        1,
        None,
        None,
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout,
                ..default()
            },
            ..default()
        },
        // Add a SpritesheetAnimation component that references our newly created animation
        SpritesheetAnimation::from_id(animation_id),
    ));

    commands.spawn(Camera2dBundle::default());
}

pub fn setup_character(
    mut commands: Commands,
    mut library: ResMut<SpritesheetLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) {
    setup_anime_periodical(
        commands,
        library,
        atlas_layouts,
        assets,
        "textures\\entities\\example_3.png".to_string(),
        8,
    );
}

pub fn setup_player(
    mut library: ResMut<SpritesheetLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
    texture_dir: String,
    number_of_frames: usize,
) -> SpriteSheetBundle {
    // Spawn a sprite using Bevy's built-in SpriteSheetBundle

    let clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(number_of_frames, 1).row(0));
    });

    let animation_id = library.new_animation(|animation| {
        animation.add_stage(clip_id.into());
    });

    let texture = assets.load(texture_dir);

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(96.0, 96.0),
        number_of_frames,
        1,
        None,
        None,
    ));

    return SpriteSheetBundle {
        texture,
        atlas: TextureAtlas {
            layout,
            ..default()
        },
        ..default()
    };
}
