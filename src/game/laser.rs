use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use super::{
    config::PositionConfig, AttackTarget, HittingEvent, OnGameScreen, Player, Scoreboard, HP,
    TOP_WALL, WALL_THICKNESS,
};

pub(super) const LASER_DURATION: f32 = 10.0;
pub(super) const LASER_COLOR: Color = Color::rgba(1.0, 0.7, 0., 0.80);
const LASER_STAR_SIZE: Vec3 = Vec3::new(1.5, 1.5, 0.);

#[derive(Component)]
pub(super) struct Laser {
    pub(super) enabled: bool,
    pub(super) duration_timer: Option<Timer>,
}

#[derive(Component)]
pub(super) struct LaserBoardUi;

#[derive(Component)]
pub(super) struct LaserRay;

#[derive(Resource)]
pub(super) struct LaserAttackTimer(Timer);

#[derive(Component)]
pub(super) struct LaserStar;

#[derive(Resource)]
pub(super) struct LaserStarGenerateTimer(Timer);

#[derive(Resource)]
pub(super) struct LaserStarVanishTimer(Timer);

pub(super) fn setup_laser(mut commands: Commands) {
    commands.insert_resource(LaserAttackTimer(Timer::from_seconds(
        0.1,
        TimerMode::Repeating,
    )));
    commands.insert_resource(LaserStarGenerateTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )));
    commands.insert_resource(LaserStarVanishTimer(Timer::from_seconds(
        1.0,
        TimerMode::Once,
    )));
}

pub(super) fn shoot_laser(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut laser_query: Query<(&mut Laser, &Transform), With<Player>>,
) {
    for (mut laser, transform) in &mut laser_query {
        if laser.enabled
            && !laser.duration_timer.as_mut().unwrap().finished()
            && keyboard_input.pressed(KeyCode::KeyL)
        {
            laser.duration_timer.as_mut().unwrap().tick(time.delta());
            commands.spawn(gen_laserray(
                &mut meshes,
                &mut materials,
                transform.translation,
            ));
        }
    }
}

pub(super) fn clear_laser(mut commands: Commands, laser_query: Query<Entity, With<LaserRay>>) {
    for entity in &laser_query {
        commands.entity(entity).despawn();
    }
}

pub(super) fn check_for_laserray_hitting(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    time: Res<Time>,
    mut laser_attack_timer: ResMut<LaserAttackTimer>,
    mut laserray_query: Query<&Transform, With<LaserRay>>,
    mut attack_target_query: Query<
        (Entity, &Transform, Option<&mut HP>, Option<&Player>),
        With<AttackTarget>,
    >,
    mut hitting_events: EventWriter<HittingEvent>,
) {
    if !laser_attack_timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for laserray_transform in &mut laserray_query {
        for (target_entity, target_transform, maybe_hp, maybe_player) in &mut attack_target_query {
            if maybe_player.is_some() {
                continue;
            }
            let laserray_shape = Aabb2d::new(
                laserray_transform.translation.truncate(),
                laserray_transform.scale.truncate() / 2.,
            );
            let bullet_target_shape = Aabb2d::new(
                target_transform.translation.truncate(),
                target_transform.scale.truncate() / 2.,
            );

            if laserray_shape.intersects(&bullet_target_shape) {
                // Bricks should be despawned and increment the scoreboard on hitting
                match maybe_hp {
                    Some(mut hp) => {
                        hp.0 = hp.0.saturating_sub(1);
                        if hp.0 <= 0 {
                            commands.entity(target_entity).despawn();
                            scoreboard.score += 1;
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
    }
}

fn gen_laserray(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    weapon_location: Vec3,
) -> impl Bundle {
    let bullet_position = weapon_location;
    let height = TOP_WALL - WALL_THICKNESS - bullet_position.y;
    return (
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            material: materials.add(LASER_COLOR).into(),
            transform: Transform::from_translation(
                bullet_position + Vec3::new(0., height / 2. + 8., 0.),
            )
            .with_scale(Vec3::new(20., height, 1.)),
            ..default()
        },
        LaserRay,
        OnGameScreen,
    );
}

pub(super) fn update_laserboard(
    laser: Query<&mut Laser, With<Player>>,
    mut query: Query<&mut Text, With<LaserBoardUi>>,
) {
    let mut text = query.single_mut();
    let laser = laser.single();
    if laser.enabled == false {
        text.sections[1].value = "N/A".to_string();
    } else {
        let timer = laser.duration_timer.as_ref().unwrap();
        let remain = (timer.remaining_secs()).floor();
        text.sections[1].value = format!("{}s", remain);
    }
}

fn gen_laser_star(asset_server: Res<AssetServer>) -> impl Bundle {
    (
        SpriteBundle {
            texture: asset_server.load("textures/entities/star.fill.png"),
            transform: Transform {
                translation: PositionConfig::default().gen().extend(0.0),
                scale: LASER_STAR_SIZE,
                ..default()
            },
            ..default()
        },
        LaserStar,
        OnGameScreen,
    )
}

pub(super) fn add_laser_star(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    laser: Query<&Laser, With<Player>>,
    time: Res<Time>,
    mut laser_star_generate_timer: ResMut<LaserStarGenerateTimer>,
    mut laser_star_vanish_timer: ResMut<LaserStarVanishTimer>,
) {
    if !laser.single().enabled {
        return;
    }
    if !laser_star_generate_timer
        .0
        .tick(time.delta())
        .just_finished()
    {
        return;
    }
    commands.spawn(gen_laser_star(asset_server));
    laser_star_vanish_timer.0.reset();
}

pub(super) fn remove_laser_star(
    mut commands: Commands,
    time: Res<Time>,
    mut laser_star_vanish_timer: ResMut<LaserStarVanishTimer>,
    laser_star_query: Query<Entity, With<LaserStar>>,
) {
    if !laser_star_vanish_timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for entity in laser_star_query.iter() {
        commands.entity(entity).despawn();
    }
}
