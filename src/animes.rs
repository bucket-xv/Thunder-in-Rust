//! This is the anime implementations of Thunder.

// TODO | Insert the animes into game.
// TODO | Write asset files.

use bevy::prelude::*;

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

pub fn setup_anime_periodical(
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    assets: &Res<AssetServer>,
    position: Vec2,
    texture_dir: String,
    number_of_frames: usize,
) -> SpriteSheetBundle {
    // Spawn a sprite using Bevy's built-in SpriteSheetBundle

    // let clip_id = library.new_clip(|clip| {
    //     clip.push_frame_indices(Spritesheet::new(number_of_frames, 1).row(0));
    // });

    // let animation_id = library.new_animation(|animation| {
    //     animation.add_stage(clip_id.into());
    // });

    let texture = assets.load(texture_dir);

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(96.0, 96.0),
        number_of_frames,
        1,
        None,
        None,
    ));

    return SpriteSheetBundle {
        transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
        texture,
        atlas: TextureAtlas {
            layout,
            ..default()
        },
        ..default()
    };
}
