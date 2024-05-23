//! This is all the default settings for game.rs.
//! It includes the default user plane, default enemy plane and default enemy generation plan.

use bevy::math::Vec2;
use bevy::render::color::Color;
use rand::{thread_rng, Rng};

use crate::game::*;

pub const ENEMY_PLANE_HP: u32 = 3;
pub const ENEMY_START_TIME: f32 = 1.0;
pub const ENEMY_GEN_INTERVAL: f32 = 2.0;
const DEFAULT_BULLET_SPEED: f32 = 350.0;
const PI: f32 = std::f32::consts::PI;

#[derive(Clone, Copy)]
pub enum PositionConfig {
    #[allow(dead_code)]
    // Deterministic postion
    Determinate(Vec2),
    // Randomized position with range of x and y
    Random(Vec2, Vec2),
}

impl PositionConfig {
    pub fn gen(self) -> Vec2 {
        let mut rng = thread_rng();
        match self {
            Self::Determinate(position) => position,
            Self::Random(x_range, y_range) => Vec2::new(
                rng.gen_range(x_range.x..x_range.y),
                rng.gen_range(y_range.x..y_range.y),
            ),
        }
    }
}
impl Default for PositionConfig {
    fn default() -> Self {
        PositionConfig::Random(
            Vec2::new(
                LEFT_WALL + GAP_BETWEEN_PLANE_AND_WALL,
                RIGHT_WALL - GAP_BETWEEN_PLANE_AND_WALL,
            ),
            Vec2::new(
                (TOP_WALL * 3.0 + BOTTOM_WALL) / 4.0,
                TOP_WALL - GAP_BETWEEN_PLANE_AND_WALL,
            ),
        )
    }
}

#[derive(Clone, Copy)]
pub enum BulletDirectionConfig {
    #[allow(dead_code)]
    Determinate(f32),
    // Random range in the form of angle
    Random(Vec2),
    Trace,
}

impl BulletDirectionConfig {
    pub fn gen(self) -> BulletDirection {
        let mut rng = thread_rng();
        match self {
            Self::Determinate(angle) => BulletDirection::Fix(angle),
            Self::Random(angle_range) => {
                BulletDirection::Fix(rng.gen_range(angle_range.x..angle_range.y))
            }
            Self::Trace => BulletDirection::Trace,
        }
    }
}
impl Default for BulletDirectionConfig {
    fn default() -> Self {
        Self::Random(Vec2::new(-3.0 * PI / 4.0, -PI / 4.0))
    }
}

pub struct EnemyConfig {
    pub position: PositionConfig,
    pub scale: Vec2,
    pub color: Color,
    pub hp: u32,
    pub weapon_type: WeaponType,
    pub bullet_color: Color,
    pub bullet_relative_position: Vec2,
    pub bullet_speed: f32,
    pub bullet_direction: BulletDirectionConfig,
    pub bullet_diameter: f32,
    pub shooting_interval: f32,
}

impl Default for EnemyConfig {
    fn default() -> Self {
        EnemyConfig {
            position: PositionConfig::default(),
            scale: PLANE_SIZE.truncate(),
            color: PLANE_COLOR,
            hp: ENEMY_PLANE_HP,
            weapon_type: WeaponType::GatlingGun,
            bullet_color: BULLET_COLOR,
            bullet_speed: DEFAULT_BULLET_SPEED,
            bullet_direction: BulletDirectionConfig::default(),
            bullet_relative_position: -BULLET_STARTING_RELATIVE_POSITION.truncate(),
            bullet_diameter: BULLET_DIAMETER,
            shooting_interval: BULLET_SHOOTING_INTERVAL,
        }
    }
}

pub enum WaveConfig {
    #[allow(dead_code)]
    Duplicate(EnemyConfig, u32),
    Detailed(Vec<EnemyConfig>),
}

impl WaveConfig {
    pub fn get_wave_len(level: u32) -> u32 {
        match level {
            1 => 2,
            2 => 4,
            3 => 5,
            4 => 5,
            5 => 5,
            _ => unimplemented!("level {} is unimplemented", level),
        }
    }
    pub fn get(level: u32, wave: u32) -> WaveConfig {
        match (level, wave) {
            //level 1
            (1, 0) => WaveConfig::Duplicate(
                EnemyConfig {
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 1,
                    ..default()
                },
                1,
            ),
            (1, 1) => WaveConfig::Duplicate(
                EnemyConfig {
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 1,
                    ..default()
                },
                2,
            ),

            //level 2
            (2, 0) => WaveConfig::Duplicate(
                EnemyConfig {
                    bullet_direction: BulletDirectionConfig::Trace,
                    ..default()
                },
                1,
            ),
            (2, 1) => WaveConfig::Duplicate(
                EnemyConfig {
                    bullet_direction: BulletDirectionConfig::Trace,
                    ..default()
                },
                2,
            ),
            (2, 2) => WaveConfig::Duplicate(
                EnemyConfig {
                    bullet_direction: BulletDirectionConfig::Trace,
                    ..default()
                },
                3,
            ),
            (2, 3) => WaveConfig::Duplicate(EnemyConfig::default(), 3),

            //level 3
            (3, 0) => WaveConfig::Duplicate(EnemyConfig::default(), 1),
            (3, 1) => WaveConfig::Duplicate(EnemyConfig::default(), 2),
            (3, 2) => WaveConfig::Duplicate(EnemyConfig::default(), 3),
            (3, 3) => WaveConfig::Duplicate(EnemyConfig::default(), 3),
            (3, 4) => WaveConfig::Duplicate(EnemyConfig::default(), 3),

            //level 4
            (4, 0) => WaveConfig::Duplicate(EnemyConfig::default(), 1),
            (4, 1) => WaveConfig::Duplicate(EnemyConfig::default(), 2),
            (4, 2) => WaveConfig::Duplicate(EnemyConfig::default(), 3),
            (4, 3) => WaveConfig::Duplicate(EnemyConfig::default(), 3),
            (4, 4) => WaveConfig::Duplicate(EnemyConfig::default(), 3),

            // level 5
            (5, 0) => WaveConfig::Duplicate(EnemyConfig::default(), 1),
            (5, 1) => WaveConfig::Duplicate(EnemyConfig::default(), 2),
            (5, 2) => WaveConfig::Duplicate(EnemyConfig::default(), 3),
            (5, 3) => WaveConfig::Duplicate(EnemyConfig::default(), 3),
            (5, 4) => WaveConfig::Duplicate(EnemyConfig::default(), 3),

            _ => unimplemented!("Level {} wave {} is not implemented", level, wave),
        }
    }
}

// pub static ENEMY_GEN: Lazy<[EnemyGenerationConfig; 3]> = Lazy::new(|| {
//     let mut rng = rand::thread_rng();
//     [
//         EnemyGenerationConfig {
//             number_of_enemies: 1,
//             weapon: Weapon {
//                 weapon_type: WeaponType::GatlingGun,
//                 bullet_config: BulletConfig {
//                     color: BULLET_COLOR,
//                     diameter: BULLET_DIAMETER,
//                     relative_position: -BULLET_STARTING_RELATIVE_POSITION,
//                     speed: DEFAULT_BULLET_SPEED
//                         * Vec2::from_angle(rng.gen_range(-3.0 * PI / 4.0..-PI / 4.0)),
//                 },
//                 shoot_timer: Timer::from_seconds(BULLET_SHOOTING_INTERVAL, TimerMode::Repeating),
//             },
//         },
//         EnemyGenerationConfig {
//             number_of_enemies: 2,
//             weapon: Weapon {
//                 weapon_type: WeaponType::GatlingGun,
//                 bullet_config: BulletConfig {
//                     color: BULLET_COLOR,
//                     diameter: BULLET_DIAMETER,
//                     relative_position: -BULLET_STARTING_RELATIVE_POSITION,
//                     speed: DEFAULT_BULLET_SPEED
//                         * Vec2::from_angle(rng.gen_range(-3.0 * PI / 4.0..-PI / 4.0)),
//                 },
//                 shoot_timer: Timer::from_seconds(BULLET_SHOOTING_INTERVAL, TimerMode::Repeating),
//             },
//         },
//         EnemyGenerationConfig {
//             number_of_enemies: 3,
//             weapon: Weapon {
//                 weapon_type: WeaponType::GatlingGun,
//                 bullet_config: BulletConfig {
//                     color: BULLET_COLOR,
//                     diameter: BULLET_DIAMETER,
//                     relative_position: -BULLET_STARTING_RELATIVE_POSITION,
//                     speed: DEFAULT_BULLET_SPEED
//                         * Vec2::from_angle(rng.gen_range(-3.0 * PI / 4.0..-PI / 4.0)),
//                 },
//                 shoot_timer: Timer::from_seconds(BULLET_SHOOTING_INTERVAL, TimerMode::Repeating),
//             },
//         },
//     ]
// });
