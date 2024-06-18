use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use super::{
    AttackTarget, HittingEvent, OnGameScreen, Player, Scoreboard, HP, TOP_WALL, WALL_THICKNESS,
};

pub const LASER_DURATION: f32 = 100.0;
pub const LASER_COLOR: Color = Color::rgba(1.0, 0.7, 0., 0.80);

#[derive(Component)]
pub struct Laser {
    pub enabled: bool,
    pub duration_timer: Option<Timer>,
}

#[derive(Component)]
pub struct LaserRay;

#[derive(Resource)]
pub struct LaserAttackTimer(Timer);

pub(super) fn setup_laser_attack_timer(mut commands: Commands) {
    commands.insert_resource(LaserAttackTimer(Timer::from_seconds(
        0.5,
        TimerMode::Repeating,
    )));
}

pub(super) fn shoot_laser(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut laser_query: Query<(&mut Laser, &Transform), With<Player>>,
) {
    for (mut laser, transform) in &mut laser_query {
        if laser.enabled && !laser.duration_timer.as_mut().unwrap().finished() {
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
