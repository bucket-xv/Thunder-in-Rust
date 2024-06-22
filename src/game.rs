//! This is the main game page of Thunder.

mod config;
pub mod esc_menu;
pub mod generator;
pub mod laser;
pub mod win_lose_screen;
use self::laser::{
    check_for_laser_star_capture, check_for_laserray_hitting, clear_laser, setup_laser,
    shoot_laser, update_laserboard, Laser, LaserBoardUi,
};

use super::{despawn_screen, GameState, Level};
// use bevy::sprite::Material2d;
use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
use core::f32::consts::PI;
use laser::{add_laser_star, remove_laser_star};
// use bevy_rand::prelude::WyRand;
// use bevy_rand::resource::GlobalEntropy;
// use rand::Rng;

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
const BULLET_SHOOTING_INTERVAL: f32 = 0.4;
const BULLET_DIAMETER: f32 = 20.;
const USER_BULLET_SPEED: f32 = 500.0;

const DEFAULT_ENEMY_BULLET_DIRECTION: f32 = -PI / 2.0;

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const HPBOARD_TEXT_PADDING: Val = Val::Px(50.0);
const LASERBOARD_TEXT_PADDING: Val = Val::Px(95.0);
const MENU_BUTTON_PADDING: Val = Val::Px(10.0);
const PLAYER_PLANE_HP: u32 = 20;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PLANE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BULLET_COLOR: Color = Color::rgb(0.7, 0.3, 0.3);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const MENU_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// This plugin will contain the game. It will focus on the state `GameState::Game`
pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Init), game_setup)
        .add_event::<HittingEvent>()
        .add_systems(OnEnter(GameState::Game), setup_laser)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                generate_enemy,
                shoot_gun,
                apply_velocity,
                clear_laser,
                move_player_plane,
                shoot_laser,
                check_for_bullet_hitting,
                check_for_laserray_hitting,
                check_for_laser_star_capture,
                play_hitting_sound,
                update_scoreboard,
                update_hpboard,
                update_laserboard,
                check_for_next_wave,
                add_laser_star,
                remove_laser_star,
            )
                // `chain`ing systems together runs them in order
                .chain()
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (button_system, game_menu_action, back_on_esc).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            OnEnter(GameState::Menu),
            (despawn_screen::<OnGameScreen>, restore_background),
        )
        .add_systems(
            OnEnter(GameState::Win),
            (despawn_screen::<OnGameScreen>, restore_background),
        )
        .add_systems(
            OnEnter(GameState::Lose),
            (despawn_screen::<OnGameScreen>, restore_background),
        )
        .add_systems(
            OnEnter(GameState::Completion),
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
struct Wave(u32);

const GAME_NORMAL_BUTTON: Color = Color::rgb(0.5, 0.5, 0.5); // Normal state: gray
const GAME_HOVERED_BUTTON: Color = Color::rgb(0.6, 0.6, 0.6); // Hovered state: slightly lighter gray
const GAME_HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.4, 0.6, 0.4); // Hovered and pressed state: greenish gray
const GAME_PRESSED_BUTTON: Color = Color::rgb(0.4, 0.7, 0.4); // Pressed state: more greenish gray

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
enum GameButtonAction {
    Paused,
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
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => GAME_PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => GAME_HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => GAME_HOVERED_BUTTON.into(),
            (Interaction::None, None) => GAME_NORMAL_BUTTON.into(),
        }
    }
}

fn game_menu_action(
    interaction_query: Query<
        (&Interaction, &GameButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, game_menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match game_menu_button_action {
                GameButtonAction::Paused => {
                    game_state.set(GameState::Stopped);
                }
            }
        }
    }
}

// Add the game's entities to our world
fn game_setup(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    level: Res<Level>,
) {
    commands.insert_resource(Scoreboard {
        // hp: PLAYER_PLANE_HP,
        score: 0,
    });
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    commands.insert_resource(EnemyGenerateTimer(Timer::from_seconds(
        config::ENEMY_START_TIME,
        TimerMode::Once,
    )));
    commands.insert_resource(Wave(0));

    // commands.spawn(Camera2dBundle::default());

    // Sound
    let hitting_sound = asset_server.load("sounds/bullet_hitting.ogg");
    commands.insert_resource(HittingSound(hitting_sound));

    // Player Plane
    let user_plane = generator::gen_user_plane(asset_server, level.0);
    commands.spawn(user_plane);

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

    // Hpboard
    commands.spawn((
        HpboardUi,
        TextBundle::from_sections([
            TextSection::new(
                "Hp: ",
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
            top: HPBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
        OnGameScreen,
    ));

    // Laserboard
    commands.spawn((
        LaserBoardUi,
        TextBundle::from_sections([
            TextSection::new(
                "Laser: ",
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
            top: LASERBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
        OnGameScreen,
    ));

    // Buttion Style
    let game_button_style = Style {
        width: Val::Px(100.0),
        height: Val::Px(45.0),
        margin: UiRect {
            top: MENU_BUTTON_PADDING,   // Adjust the top margin
            right: MENU_BUTTON_PADDING, // Adjust the right margin
            ..default()
        },
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let game_button_text_style = TextStyle {
        font_size: 40.0,
        color: MENU_COLOR,
        ..default()
    };

    // Menu button
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexEnd,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            parent // turn to stopped menu
                .spawn((
                    ButtonBundle {
                        style: game_button_style.clone(),
                        background_color: GAME_NORMAL_BUTTON.into(),
                        ..default()
                    },
                    GameButtonAction::Paused,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "MENU",
                        game_button_text_style.clone(),
                    ));
                });
        });

    // Walls
    commands.spawn((WallBundle::new(WallLocation::Left), OnGameScreen));
    commands.spawn((WallBundle::new(WallLocation::Right), OnGameScreen));
    commands.spawn((WallBundle::new(WallLocation::Bottom), OnGameScreen));
    commands.spawn((WallBundle::new(WallLocation::Top), OnGameScreen));

    // Start the game
    game_state.set(GameState::Game);
}

#[derive(Component)]
struct Plane;

#[derive(Bundle)]
struct PlaneBundle {
    plane: Plane,
    gun: GatlingGun,
    laser: Laser,
    on_game_screen: OnGameScreen,
    hp: HP,
    bullet_target: AttackTarget,
    sprite_bundle: SpriteBundle,
}

#[derive(Bundle)]
pub struct PlayerPlaneBundle {
    plane_bundle: PlaneBundle,
    player: Player,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    plane_bundle: PlaneBundle,
    enemy: Enemy,
}

#[derive(Component, Clone)]
pub struct GatlingGun {
    bullet_config: BulletConfig,
    shoot_timer: Timer,
}

#[derive(Clone, Copy)]
struct BulletConfig {
    color: Color,
    relative_position: Vec3,
    diameter: f32,
    speed: f32,
    direction: BulletDirection,
}

#[derive(Clone, Copy)]
enum BulletDirection {
    Fix(f32),
    Trace,
}

#[derive(Component)]
struct Bullet;

// Bullet will despawn when hitting the entity with the BulletTarget component
#[derive(Component)]
struct AttackTarget;

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
    HitLaserStar,
}

#[derive(Resource)]
struct HittingSound(Handle<AudioSource>);

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: AttackTarget,
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
            collider: AttackTarget,
        }
    }
}

// This resource tracks the game's score
#[derive(Resource)]
struct Scoreboard {
    // hp: u32,
    score: u32,
}

#[derive(Component)]
struct ScoreboardUi;

#[derive(Component)]
struct HpboardUi;

fn generate_enemy(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemyGenerateTimer>,
    wave: ResMut<Wave>,
    level: Res<Level>,
    // rng: ResMut<GlobalEntropy<WyRand>>,
) {
    if timer.tick(time.delta()).just_finished() {
        timer.0.reset();
        timer.0.pause();
        let vec = generator::gen_wave(level.0, wave.0);
        for plane in vec {
            commands.spawn(plane);
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

    if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
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
    let display = scoreboard.score.to_string();
    // display.push_str(" | HP: ");
    // display.push_str(&scoreboard.hp.to_string());
    text.sections[1].value = display;
}

fn update_hpboard(
    query_player_plane: Query<&HP, With<Player>>,
    mut query: Query<&mut Text, With<HpboardUi>>,
) {
    let mut text = query.single_mut();
    let hp = match query_player_plane.is_empty() {
        true => 0,
        false => query_player_plane.single().0,
    };
    text.sections[1].value = hp.to_string();
}

fn shoot_gun(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut enemy_gun_query: Query<(&mut GatlingGun, &Transform), (With<Enemy>, Without<Player>)>,
    mut player_gun_query: Query<(&mut GatlingGun, &Transform), With<Player>>,
) {
    let player_plane_loc = player_gun_query.single().1.translation;
    for (mut gun, gun_transform) in &mut enemy_gun_query {
        if gun.shoot_timer.tick(time.delta()).just_finished() {
            commands.spawn(generator::gen_bullet(
                &mut meshes,
                &mut materials,
                &gun,
                gun_transform.translation,
                player_plane_loc,
            ));
        }
    }
    for (mut gun, gun_transform) in &mut player_gun_query {
        if gun.shoot_timer.tick(time.delta()).just_finished() {
            commands.spawn(generator::gen_bullet(
                &mut meshes,
                &mut materials,
                &gun,
                gun_transform.translation,
                player_plane_loc,
            ));
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
            game_state.set(GameState::Stopped);
        }
    }
}

fn restore_background(mut commands: Commands) {
    commands.insert_resource(ClearColor::default());
}

fn check_for_bullet_hitting(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut attack_target_query: Query<
        (Entity, &Transform, Option<&mut HP>, Option<&Player>),
        With<AttackTarget>,
    >,
    mut hitting_events: EventWriter<HittingEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let mut despawn_bullet = false;
        for (target_entity, transform, maybe_hp, maybe_player) in &mut attack_target_query {
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
                                    // TODO: Defeat
                                    game_state.set(GameState::Lose);
                                }
                                None => {
                                    scoreboard.score += 1;
                                }
                            }
                        }
                        // if maybe_player.is_some() {
                        //     scoreboard.hp = scoreboard.hp.saturating_sub(1);
                        // }
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
            HittingEvent::HitLaserStar => {
                commands.spawn(AudioBundle {
                    source: sound.0.clone(),
                    // auto-despawn the entity when playback finishes
                    settings: PlaybackSettings::DESPAWN,
                });
                break;
            }
        }
    }
    // This prevents events staying active on the next frame.
    hitting_events.clear();
}

fn check_for_next_wave(
    plane: Query<&Enemy>,
    mut timer: ResMut<EnemyGenerateTimer>,
    mut wave: ResMut<Wave>,
    mut game_state: ResMut<NextState<GameState>>,
    level: Res<Level>,
) {
    if plane.iter().next().is_none() && timer.paused() {
        info!("All enemies are destroyed. Next wave is coming.");
        wave.0 += 1;

        *timer = EnemyGenerateTimer(Timer::from_seconds(
            config::ENEMY_GEN_INTERVAL,
            TimerMode::Once,
        ));
        if wave.0 >= config::WaveConfig::get_wave_len(level.0) {
            if level.0 == 5 {
                game_state.set(GameState::Completion);
            } else {
                game_state.set(GameState::Win);
            }
        }
    }
}
