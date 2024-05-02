//! This is the main game page of Thunder.

mod config;

use super::{despawn_screen, GameState};
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use rand::Rng;
//use super::{DisplayQuality, Volume};

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
const PLANE_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const GAP_BETWEEN_PLANE_AND_WALL: f32 = 60.0;
const PLAYER_PLANE_SPEED: f32 = 300.0;

// How close can the plane get to the wall
const PLANE_PADDING: f32 = 10.0;

// We set the z-value of the bullet to 1 so it renders on top in the case of overlapping sprites.
const BULLET_STARTING_RELATIVE_POSITION: Vec3 = Vec3::new(0.0, 50.0, 0.0);
const BULLET_SHOOTING_INTERVAL: f32 = 0.2;
const BULLET_DIAMETER: f32 = 20.;

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const PLAYER_PLANE_HP: u32 = 10;
const ENEMY_PLANE_HP: u32 = 1;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PLANE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BULLET_COLOR: Color = Color::rgb(0.7, 0.3, 0.3);
// const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
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
                generate_enemy,
                shoot_bullets,
                apply_velocity,
                move_player_plane,
                check_for_hitting,
                play_hitting_sound,
                check_for_next_wave,
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

#[derive(Component)]
struct Enemy;

#[derive(Resource, Deref, DerefMut)]
struct EnemyGenerateTimer(Timer);

#[derive(Resource, Deref, DerefMut)]
struct EnemyWaveIndex(u32);

// Add the game's entities to our world
fn game_setup(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Scoreboard { hp: 10, score: 0 });
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    commands.insert_resource(EnemyGenerateTimer(Timer::from_seconds(
        config::ENEMY_START_TIME,
        TimerMode::Once,
    )));
    commands.insert_resource(EnemyWaveIndex(0));

    // commands.spawn(Camera2dBundle::default());

    // Sound
    let hitting_sound = asset_server.load("sounds/bullet_hitting.ogg");
    commands.insert_resource(HittingSound(hitting_sound));

    // Player Plane
    let plane_y = BOTTOM_WALL + GAP_BETWEEN_PLANE_AND_WALL;

    commands.spawn((
        PlaneBundle {
            plane: Plane,
            sprite_bundle: SpriteBundle {
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
            hp: HP(PLAYER_PLANE_HP),
            on_game_screen: OnGameScreen,
            weapon: Weapon {
                weapon_type: WeaponType::GatlingGun,
                bullet_config: BulletConfig {
                    color: BULLET_COLOR,
                    diameter: BULLET_DIAMETER,
                    relative_position: BULLET_STARTING_RELATIVE_POSITION,
                    speed: Vec2::new(0.0, config::USER_BULLET_SPEED),
                },
                shoot_timer: Timer::from_seconds(BULLET_SHOOTING_INTERVAL, TimerMode::Repeating),
            },
            bullet_target: BulletTarget,
        },
        Player,
    ));

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
}

#[derive(Component)]
struct Plane;

#[derive(Bundle)]
struct PlaneBundle {
    plane: Plane,
    weapon: Weapon,
    on_game_screen: OnGameScreen,
    hp: HP,
    bullet_target: BulletTarget,
    sprite_bundle: SpriteBundle,
}

#[derive(Component, Clone)]
struct Weapon {
    weapon_type: WeaponType,
    bullet_config: BulletConfig,
    shoot_timer: Timer,
}
#[derive(Clone)]
enum WeaponType {
    GatlingGun,
    Laser,
}
#[derive(Clone)]
struct BulletConfig {
    color: Color,
    relative_position: Vec3,
    diameter: f32,
    speed: Vec2,
}

#[derive(Component)]
struct Bullet;

// Bullet will despawn when hitting the entity with the BulletTarget component
#[derive(Component)]
struct BulletTarget;

/// HP will decrease when hitted with Bullet
/// When HP is 0, the entity with HP component will be despawned
#[derive(Component)]
struct HP(u32);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Event)]
enum HittingEvent {
    HitWall,
    HitPlane,
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
    hp: u32,
    score: u32,
}

#[derive(Component)]
struct ScoreboardUi;

fn generate_enemy(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemyGenerateTimer>,
    enemy_wave_index: ResMut<EnemyWaveIndex>,
) {
    let mut rng = rand::thread_rng();

    if timer.tick(time.delta()).just_finished() {
        timer.0.reset();
        timer.0.pause();
        for _ in 0..config::ENEMY_GEN[enemy_wave_index.0 as usize].number_of_enemies {
            let plane_x = rng.gen_range(
                LEFT_WALL + GAP_BETWEEN_PLANE_AND_WALL..RIGHT_WALL - GAP_BETWEEN_PLANE_AND_WALL,
            );
            let plane_y = TOP_WALL - GAP_BETWEEN_PLANE_AND_WALL;
            commands.spawn((
                PlaneBundle {
                    sprite_bundle: SpriteBundle {
                        transform: Transform {
                            translation: Vec3::new(plane_x, plane_y, 0.0),
                            scale: PLANE_SIZE,
                            ..default()
                        },
                        sprite: Sprite {
                            color: PLANE_COLOR,
                            ..default()
                        },
                        ..default()
                    },
                    plane: Plane,
                    weapon: config::ENEMY_GEN[enemy_wave_index.0 as usize]
                        .weapon
                        .clone(),
                    bullet_target: BulletTarget,
                    on_game_screen: OnGameScreen,
                    hp: HP(ENEMY_PLANE_HP),
                },
                Enemy,
            ));
        }
    }
}

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
    let mut display = scoreboard.score.to_string();
    display.push_str(" | HP: ");
    display.push_str(&scoreboard.hp.to_string());
    text.sections[1].value = display;
}

fn shoot_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut weapon_query: Query<(&mut Weapon, &mut Transform), With<Plane>>,
) {
    for (mut weapon, weapon_transform) in &mut weapon_query {
        if weapon.shoot_timer.tick(time.delta()).just_finished() {
            let bullet_position =
                weapon_transform.translation + weapon.bullet_config.relative_position;

            match weapon.weapon_type {
                WeaponType::GatlingGun => commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Circle::default()).into(),
                        material: materials.add(weapon.bullet_config.color).into(),
                        transform: Transform::from_translation(bullet_position)
                            .with_scale(Vec2::splat(weapon.bullet_config.diameter).extend(1.)),
                        ..default()
                    },
                    Velocity(weapon.bullet_config.speed),
                    Bullet,
                    OnGameScreen,
                )),
                WeaponType::Laser => {
                    //TODO: Implement laser
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(Circle::default()).into(),
                            material: materials.add(weapon.bullet_config.color).into(),
                            transform: Transform::from_translation(bullet_position)
                                .with_scale(Vec2::splat(weapon.bullet_config.diameter).extend(1.)),
                            ..default()
                        },
                        OnGameScreen,
                    ))
                }
            };
        }
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
    mut bullet_target_query: Query<
        (Entity, &Transform, Option<&mut HP>, Option<&Player>),
        With<BulletTarget>,
    >,
    mut hitting_events: EventWriter<HittingEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let mut despawn_bullet = false;
        for (target_entity, transform, maybe_hp, maybe_player) in &mut bullet_target_query {
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
                match maybe_hp {
                    Some(mut hp) => {
                        hp.0 = hp.0.saturating_sub(1);
                        if hp.0 == 0 {
                            commands.entity(target_entity).despawn();
                            match maybe_player {
                                Some(_) => {
                                    game_state.set(GameState::Menu);
                                }
                                None => {
                                    scoreboard.score += 1;
                                }
                            }
                        }
                        if maybe_player.is_some() {
                            scoreboard.hp -= 1;
                        }
                        hitting_events.send(HittingEvent::HitPlane)
                    }
                    // Walls should not be despawned
                    None => hitting_events.send(HittingEvent::HitWall),
                };
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
            HittingEvent::HitPlane => {
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

fn check_for_next_wave(
    plane: Query<&Enemy>,
    mut timer: ResMut<EnemyGenerateTimer>,
    mut enemy_wave_index: ResMut<EnemyWaveIndex>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if plane.iter().next().is_none() && timer.paused() {
        info!("All enemies are destroyed. Next wave is coming.");
        enemy_wave_index.0 += 1;

        *timer = EnemyGenerateTimer(Timer::from_seconds(
            config::ENEMY_GEN_INTERVAL,
            TimerMode::Once,
        ));
        if enemy_wave_index.0 as usize >= config::ENEMY_GEN.len() {
            game_state.set(GameState::Menu);
        }
    }
}
