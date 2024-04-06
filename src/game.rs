//! A simplified implementation of the classic game "Breakout".

use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use super::{despawn_screen, GameState};
//use super::{DisplayQuality, Volume};

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
const PLANE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const GAP_BETWEEN_PLANE_AND_FLOOR: f32 = 60.0;
const PLAYER_PLANE_SPEED: f32 = 300.0;

// How close can the plane get to the wall
const PLANE_PADDING: f32 = 10.0;

// We set the z-value of the bullet to 1 so it renders on top in the case of overlapping sprites.
const BULLET_STARTING_RELATIVE_POSITION: Vec3 = Vec3::new(0.0, 50.0, 0.0);
const BULLET_SHOOTING_INTERVAL: f32 = 0.2;
const BULLET_DIAMETER: f32 = 20.;
const BULLET_SPEED: f32 = 600.0;

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
// These values are exact
const GAP_BETWEEN_PLANE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
// These values are lower bounds, as the number of bricks is computed
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PLANE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BULLET_COLOR: Color = Color::rgb(0.7, 0.3, 0.3);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

// This plugin will contain the game. It will focus on the state `GameState::Game`
pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_setup)
        .add_event::<HittingEvent>()
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                shoot_bullets,
                apply_velocity,
                move_player_plane,
                check_for_hitting,
                play_hitting_sound,
            )
                // `chain`ing systems together runs them in order
                .chain()
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (update_scoreboard, back_on_esc).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            OnExit(GameState::Game),
            (despawn_screen::<OnGameScreen>, restore_background),
        );
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct Player;

#[derive(Resource, Deref, DerefMut)]
struct ShootTimer(Timer);

// Add the game's entities to our world
fn game_setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Scoreboard { score: 0 });
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    commands.insert_resource(ShootTimer(Timer::from_seconds(
        BULLET_SHOOTING_INTERVAL,
        TimerMode::Repeating,
    )));

    // commands.spawn(Camera2dBundle::default());

    // Sound
    let hitting_sound = asset_server.load("sounds/bullet_hitting.ogg");
    commands.insert_resource(HittingSound(hitting_sound));

    // Player Plane
    let plane_y = BOTTOM_WALL + GAP_BETWEEN_PLANE_AND_FLOOR;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, plane_y, 0.0),
                scale: PLANE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PLANE_COLOR,
                ..default()
            },
            ..default()
        },
        Plane,
        Player,
        BulletTarget,
        OnGameScreen,
    ));

    // Ball
    // commands.spawn((
    //     MaterialMesh2dBundle {
    //         mesh: meshes.add(Circle::default()).into(),
    //         material: materials.add(BALL_COLOR),
    //         transform: Transform::from_translation(BALL_STARTING_POSITION)
    //             .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
    //         ..default()
    //     },
    //     Ball,
    //     Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    //     OnGameScreen,
    // ));

    // Scoreboard
    commands.spawn((
        ScoreboardUi,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
        OnGameScreen,
    ));

    // Walls
    commands.spawn((WallBundle::new(WallLocation::Left), OnGameScreen));
    commands.spawn((WallBundle::new(WallLocation::Right), OnGameScreen));
    commands.spawn((WallBundle::new(WallLocation::Bottom), OnGameScreen));
    commands.spawn((WallBundle::new(WallLocation::Top), OnGameScreen));

    // Bricks
    let total_width_of_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bottom_edge_of_bricks = plane_y + GAP_BETWEEN_PLANE_AND_BRICKS;
    let total_height_of_bricks = TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

    assert!(total_width_of_bricks > 0.0);
    assert!(total_height_of_bricks > 0.0);

    // Given the space available, compute how many rows and columns of bricks we can fit
    let n_columns = (total_width_of_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_rows = (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    // Because we need to round the number of columns,
    // the space on the top and sides of the bricks only captures a lower bound, not an exact value
    let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
    let left_edge_of_bricks = center_of_bricks
        // Space taken up by the bricks
        - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
        // Space taken up by the gaps
        - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

    // In Bevy, the `translation` of an entity describes the center point,
    // not its bottom-left corner
    let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
    let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let brick_position = Vec2::new(
                offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            // brick
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: BRICK_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Brick,
                BulletTarget,
                OnGameScreen,
            ));
        }
    }
}

#[derive(Component)]
struct Plane;

#[derive(Component)]
struct Bullet;

// Bullet will despawn when hitting the entity with the BulletTarget component
#[derive(Component)]
struct BulletTarget;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Event)]
enum HittingEvent {
    HitWall,
    HitBrick,
}
#[derive(Component)]
struct Brick;

#[derive(Resource)]
struct HittingSound(Handle<AudioSource>);

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: BulletTarget,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: BulletTarget,
        }
    }
}

// This resource tracks the game's score
#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

#[derive(Component)]
struct ScoreboardUi;

fn move_player_plane(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut plane_transform = query.single_mut();
    let mut direction = Vec3::new(0.0, 0.0, 0.0);

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    assert_eq!(plane_transform.translation.z, 0.0);
    // Calculate the new horizontal plane position based on player input
    let new_plane_position =
        plane_transform.translation + direction * PLAYER_PLANE_SPEED * time.delta_seconds();

    // Update the plane position,
    // making sure it doesn't cause the plane to leave the arena
    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PLANE_SIZE.x / 2.0 + PLANE_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PLANE_SIZE.x / 2.0 - PLANE_PADDING;
    let down_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PLANE_SIZE.x / 2.0 + PLANE_PADDING;
    let up_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PLANE_SIZE.x / 2.0 - PLANE_PADDING;

    plane_transform.translation = new_plane_position.clamp(
        Vec3::new(left_bound, down_bound, 0.0),
        Vec3::new(right_bound, up_bound, 0.0),
    );
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

fn shoot_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ShootTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<&Transform, With<Player>>,
) {
    if timer.tick(time.delta()).just_finished() {
        let plane_transform = query.single();
        let bullet_position = plane_transform.translation + BULLET_STARTING_RELATIVE_POSITION;

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(BULLET_COLOR).into(),
                transform: Transform::from_translation(bullet_position)
                    .with_scale(Vec2::splat(BULLET_DIAMETER).extend(1.)),
                ..default()
            },
            Velocity(Vec2::new(0.0, BULLET_SPEED)),
            Bullet,
            OnGameScreen,
        ));
    }
}

fn back_on_esc(
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (_, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            game_state.set(GameState::Menu);
        }
    }
}

fn restore_background(mut commands: Commands) {
    commands.insert_resource(ClearColor::default());
}

fn check_for_hitting(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    bullet_target_query: Query<(Entity, &Transform, Option<&Brick>), With<BulletTarget>>,
    mut hitting_events: EventWriter<HittingEvent>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let mut despawn_bullet = false;
        for (target_entity, transform, maybe_brick) in bullet_target_query.iter() {
            let bullet_shape = BoundingCircle::new(
                bullet_transform.translation.truncate(),
                BULLET_DIAMETER / 2.,
            );
            let bullet_target_shape = Aabb2d::new(
                transform.translation.truncate(),
                transform.scale.truncate() / 2.,
            );

            if bullet_shape.intersects(&bullet_target_shape) {
                // Sends a hitting event so that other systems can react to the hitting
                despawn_bullet = true;

                // Bricks should be despawned and increment the scoreboard on hitting
                if maybe_brick.is_some() {
                    scoreboard.score += 1;
                    commands.entity(target_entity).despawn();
                    hitting_events.send(HittingEvent::HitBrick);
                }
                // Walls should not be despawned
                else {
                    hitting_events.send(HittingEvent::HitWall);
                }
            }
        }
        if despawn_bullet {
            commands.entity(bullet_entity).despawn();
        }
    }
}

fn play_hitting_sound(
    mut commands: Commands,
    mut hitting_events: EventReader<HittingEvent>,
    sound: Res<HittingSound>,
) {
    // Play a sound once per frame if a hitting occurred.
    for event in hitting_events.read() {
        match event {
            HittingEvent::HitBrick => {
                commands.spawn(AudioBundle {
                    source: sound.0.clone(),
                    // auto-despawn the entity when playback finishes
                    settings: PlaybackSettings::DESPAWN,
                });
                break;
            }
            HittingEvent::HitWall => {}
        }
    }
    // This prevents events staying active on the next frame.
    hitting_events.clear();
}

// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
// enum Collision {
//     Left,
//     Right,
//     Top,
//     Bottom,
// }

// Returns `Some` if `bullet` collides with `wall`. The returned `Collision` is the
// side of `wall` that `bullet` hit.
// fn collide_with_side(bullet: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
//     if !bullet.intersects(&wall) {
//         return None;
//     }

//     let closest = wall.closest_point(bullet.center());
//     let offset = bullet.center() - closest;
//     let side = if offset.x.abs() > offset.y.abs() {
//         if offset.x < 0. {
//             Collision::Left
//         } else {
//             Collision::Right
//         }
//     } else if offset.y > 0. {
//         Collision::Top
//     } else {
//         Collision::Bottom
//     };

//     Some(side)
// }
