//! This is all the default settings for game.rs.
//! It includes the default user plane, default enemy plane and default enemy generation plan.

use super::*;
use once_cell::sync::Lazy;

pub const ENEMY_START_TIME: f32 = 3.0;
pub const ENEMY_GEN_INTERVAL: f32 = 5.0;
pub const USER_BULLET_SPEED: f32 = 500.0;
const DEFAULT_BULLET_SPEED: f32 = 500.0;
const PI: f32 = std::f32::consts::PI;

pub struct EnemyGenerationConfig {
    pub number_of_enemies: u32,
    pub weapon: Weapon,
}

pub static ENEMY_GEN: Lazy<[EnemyGenerationConfig; 3]> = Lazy::new(|| {
    let mut rng = rand::thread_rng();
    [
        EnemyGenerationConfig {
            number_of_enemies: 1,
            weapon: Weapon {
                weapon_type: WeaponType::GatlingGun,
                bullet_config: BulletConfig {
                    color: BULLET_COLOR,
                    diameter: BULLET_DIAMETER,
                    relative_position: -BULLET_STARTING_RELATIVE_POSITION,
                    speed: DEFAULT_BULLET_SPEED
                        * Vec2::from_angle(rng.gen_range(-3.0 * PI / 4.0..-PI / 4.0)),
                },
                shoot_timer: Timer::from_seconds(BULLET_SHOOTING_INTERVAL, TimerMode::Repeating),
            },
        },
        EnemyGenerationConfig {
            number_of_enemies: 2,
            weapon: Weapon {
                weapon_type: WeaponType::GatlingGun,
                bullet_config: BulletConfig {
                    color: BULLET_COLOR,
                    diameter: BULLET_DIAMETER,
                    relative_position: -BULLET_STARTING_RELATIVE_POSITION,
                    speed: DEFAULT_BULLET_SPEED
                        * Vec2::from_angle(rng.gen_range(-3.0 * PI / 4.0..-PI / 4.0)),
                },
                shoot_timer: Timer::from_seconds(BULLET_SHOOTING_INTERVAL, TimerMode::Repeating),
            },
        },
        EnemyGenerationConfig {
            number_of_enemies: 3,
            weapon: Weapon {
                weapon_type: WeaponType::GatlingGun,
                bullet_config: BulletConfig {
                    color: BULLET_COLOR,
                    diameter: BULLET_DIAMETER,
                    relative_position: -BULLET_STARTING_RELATIVE_POSITION,
                    speed: DEFAULT_BULLET_SPEED
                        * Vec2::from_angle(rng.gen_range(-3.0 * PI / 4.0..-PI / 4.0)),
                },
                shoot_timer: Timer::from_seconds(BULLET_SHOOTING_INTERVAL, TimerMode::Repeating),
            },
        },
    ]
});
