use crate::game::config::{EnemyConfig, WaveConfig};
use crate::game::*;
use bevy::prelude::*;
use bevy_rand::prelude::GlobalEntropy;
use bevy_rand::prelude::WyRand;
// use rand::{thread_rng, Rng};

pub fn gen_wave(level: u32, wave: u32, mut rng: ResMut<GlobalEntropy<WyRand>>) -> Vec<EnemyBundle> {
    let config = WaveConfig::get(level, wave);
    match config {
        WaveConfig::Duplicate(enemy_config, enemy_num) => (0..enemy_num)
            .map(|_| gen_enemy(&enemy_config, &mut rng))
            .collect(),

        WaveConfig::Detailed(enemy_configs) => enemy_configs
            .iter()
            .map(|enemy_config| gen_enemy(&enemy_config, &mut rng))
            .collect(),
    }
}

fn gen_enemy(
    enemy_config: &EnemyConfig,
    mut _rng: &mut ResMut<GlobalEntropy<WyRand>>,
) -> EnemyBundle {
    // let plane_x = rng
    //     .gen_range(LEFT_WALL + GAP_BETWEEN_PLANE_AND_WALL..RIGHT_WALL - GAP_BETWEEN_PLANE_AND_WALL);
    // let plane_y = TOP_WALL - GAP_BETWEEN_PLANE_AND_WALL;

    EnemyBundle {
        plane_bundle: PlaneBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: enemy_config.position.gen().extend(0.0),
                    scale: enemy_config.scale.extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: enemy_config.color,
                    ..default()
                },
                ..default()
            },
            plane: Plane,
            weapon: Weapon {
                weapon_type: enemy_config.weapon_type,
                bullet_config: BulletConfig {
                    color: enemy_config.color,
                    relative_position: enemy_config.bullet_relative_position.extend(0.0),
                    diameter: enemy_config.bullet_diameter,
                    speed: enemy_config.bullet_speed.gen(),
                },
                shoot_timer: Timer::from_seconds(
                    enemy_config.shooting_interval,
                    TimerMode::Repeating,
                ),
            },
            bullet_target: BulletTarget,
            on_game_screen: OnGameScreen,
            hp: HP(enemy_config.hp),
        },
        enemy: Enemy {},
    }
}
