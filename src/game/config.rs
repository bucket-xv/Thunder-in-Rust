//! This is all the default settings for game.rs.
//! It includes the default user plane, default enemy plane and default enemy generation plan.

use bevy::math::Vec2;
use bevy::render::color::Color;
use rand::{thread_rng, Rng};

use crate::game::*;

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
            1 => 4,
            2 => 4,
            3 => 5,
            4 => 5,
            5 => 6,
            _ => unimplemented!("level {} is unimplemented", level),
        }
    }
    pub fn get(level: u32, wave: u32) -> WaveConfig {
        match (level, wave) {
            //level 1
            (1, 0) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.5 * RIGHT_WALL , 0.5 * TOP_WALL)),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    hp: 10,
                    bullet_speed: 150.0,
                    shooting_interval: 2.0,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.5 * LEFT_WALL , 0.5 * TOP_WALL)),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    hp: 10,
                    bullet_speed: 150.0,
                    shooting_interval: 2.0,
                    ..default()
                },
            ]  
            ),
            (1, 1) => WaveConfig::Duplicate(
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.0 , 0.5 * TOP_WALL)),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    hp: 80,
                    bullet_speed: 200.0,
                    shooting_interval: 1.3,
                    ..default()
                },
                1,
            ),
            (1, 2) => WaveConfig::Duplicate(
                EnemyConfig {
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 30,
                    bullet_speed: 200.0,
                    shooting_interval: 1.0,
                    ..default()
                },
                1,
            ),
            (1, 3) => WaveConfig::Duplicate(
                EnemyConfig {
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 40,
                    bullet_speed: 240.0,
                    shooting_interval: 1.0,
                    ..default()
                },
                2,
            ),

            //level 2
            (2, 0) => WaveConfig::Duplicate(
                EnemyConfig {
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    hp: 100,
                    bullet_speed: 250.0 ,
                    shooting_interval: 1.0,
                    ..default()
                },
                1,
            ),
            (2, 1) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * LEFT_WALL, 0.2 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 100,
                    bullet_speed: 300.0 ,
                    shooting_interval: 1.0,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * RIGHT_WALL, 0.8 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 100,
                    bullet_speed: 300.0 ,
                    shooting_interval: 1.0,
                    ..default()
                }
            ]
            ),
            (2, 2) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * LEFT_WALL, 0.4 * LEFT_WALL) , 
                        Vec2::new(0.4 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 80,
                    bullet_speed: 300.0 ,
                    shooting_interval: 1.4,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * LEFT_WALL, 0.2 * RIGHT_WALL) , 
                        Vec2::new(0.4 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    hp: 120,
                    bullet_speed: 300.0 ,
                    shooting_interval: 0.8,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.4 * RIGHT_WALL, 0.8 * RIGHT_WALL) , 
                        Vec2::new(0.4 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 80,
                    bullet_speed: 300.0 ,
                    shooting_interval: 1.4,
                    ..default()
                }
            ]
            ),
            (2, 3) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.95 * LEFT_WALL, 0.65 * LEFT_WALL) , 
                        Vec2::new(0.4 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 40,
                    shooting_interval: 1.2,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.55 * LEFT_WALL, 0.3 * LEFT_WALL) , 
                        Vec2::new(0.4 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 80,
                    shooting_interval: 1.2,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * LEFT_WALL, 0.2 * RIGHT_WALL) , 
                        Vec2::new(0.4 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    hp: 120,
                    shooting_interval: 0.8,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.3 * RIGHT_WALL, 0.55 * RIGHT_WALL) , 
                        Vec2::new(0.4 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 80,
                    shooting_interval: 1.2,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.65 * RIGHT_WALL, 0.95 * RIGHT_WALL) , 
                        Vec2::new(0.4 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 40,
                    shooting_interval: 1.2,
                    ..default()
                }
            ]
            ),

            //level 3
            (3, 0) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * LEFT_WALL, 0.4 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    hp: 100,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * LEFT_WALL, 0.2 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    hp: 100,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.4 * RIGHT_WALL, 0.8 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    hp: 100,
                    ..default()
                }
            ]
            ),
            (3, 1) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * LEFT_WALL, 0.2 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 100,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * RIGHT_WALL, 0.8 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 100,
                    ..default()
                }
            ]
            ),
            (3, 2) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * LEFT_WALL, 0.4 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 80,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * LEFT_WALL, 0.2 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 80,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.4 * RIGHT_WALL, 0.8 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 80,
                    ..default()
                }
            ]
            ),
            (3, 3) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * LEFT_WALL, 0.2 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 80,
                    bullet_speed : 400.0,
                    shooting_interval: 0.4,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * RIGHT_WALL, 0.8 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 80,
                    bullet_speed : 400.0,
                    shooting_interval: 0.4,
                    ..default()
                }
            ]
            ),
            (3, 4) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.9 * RIGHT_WALL , 0.0)),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 20,
                    bullet_speed: 100.0,
                    shooting_interval: 0.4,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.72 * RIGHT_WALL , 0.4 * TOP_WALL)),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 40,
                    bullet_speed: 150.0,
                    shooting_interval: 0.6,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.54 * RIGHT_WALL , 0.6 * TOP_WALL)),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 80,
                    bullet_speed: 200.0,
                    shooting_interval: 0.8,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.38 * RIGHT_WALL , 0.75 * TOP_WALL)),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 120,
                    bullet_speed: 300.0,
                    shooting_interval: 0.9,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.18 * RIGHT_WALL , 0.85 * TOP_WALL)),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 160,
                    bullet_speed: 350.0,
                    shooting_interval: 1.0,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.0 , 0.9 * TOP_WALL)),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    hp: 200,
                    bullet_speed: 400.0,
                    shooting_interval: 0.4,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.18 * LEFT_WALL , 0.85 * TOP_WALL)),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 160,
                    bullet_speed: 350.0,
                    shooting_interval: 1.0,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.36 * LEFT_WALL , 0.75 * TOP_WALL)),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 120,
                    bullet_speed: 300.0,
                    shooting_interval: 0.9,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.54 * LEFT_WALL , 0.6 * TOP_WALL)),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 80,
                    bullet_speed: 200.0,
                    shooting_interval: 0.8,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.72 * LEFT_WALL , 0.4 * TOP_WALL)),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 40,
                    bullet_speed: 150.0,
                    shooting_interval: 0.6,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Determinate(Vec2::new(0.9 * LEFT_WALL , 0.0)),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 20,
                    bullet_speed: 100.0,
                    shooting_interval: 0.4,
                    ..default()
                }
            ]
            ),

            //level 4
            (4, 0) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * LEFT_WALL, 0.2 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.4,
                    hp: 100,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * RIGHT_WALL, 0.8 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.4,
                    hp: 100,
                    ..default()
                }
            ]
            ),
            (4, 1) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * LEFT_WALL, 0.4 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 100,
                    bullet_speed : 400.0,
                    shooting_interval : 0.4,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * LEFT_WALL, 0.2 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    hp: 150,
                    bullet_speed : 500.0,
                    shooting_interval : 0.6,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.4 * RIGHT_WALL, 0.8 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 400.0,
                    shooting_interval : 0.4,
                    hp: 100,
                    ..default()
                }
            ]
            ),
            (4, 2) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.9 * LEFT_WALL, 0.7 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 400.0,
                    shooting_interval : 0.4,
                    hp: 80,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.6 * LEFT_WALL, 0.4 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 400.0,
                    shooting_interval : 0.4,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * LEFT_WALL, 0.2 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 400.0,
                    shooting_interval : 0.4,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.4 * RIGHT_WALL, 0.6 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 400.0,
                    shooting_interval : 0.4,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.7 * RIGHT_WALL, 0.9 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 400.0,
                    shooting_interval : 0.4,
                    hp: 120,
                    ..default()
                }
            ]
            ),
            (4, 3) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * RIGHT_WALL, 0.9 * RIGHT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.6 * RIGHT_WALL, 0.7 * RIGHT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.4 * RIGHT_WALL, 0.5 * RIGHT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * RIGHT_WALL, 0.3 * RIGHT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.0 , 0.1 * RIGHT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.1 * LEFT_WALL, 0.0) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.3 * LEFT_WALL, 0.2 * LEFT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.5 * LEFT_WALL, 0.4 * LEFT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.7 * LEFT_WALL, 0.6 * LEFT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.9 * LEFT_WALL, 0.8 * LEFT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Determinate(1.5 * PI),
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
            ]  
            ),
            (4, 4) => WaveConfig::Duplicate(
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.1 * LEFT_WALL, 0.1 * RIGHT_WALL) , 
                        Vec2::new(0.7 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 400.0,
                    shooting_interval : 0.4,
                    hp: 1500,
                    ..default()
                }, 
                1
            ),

            // level 5
            (5, 0) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * LEFT_WALL, 0.2 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 666.0,
                    shooting_interval : 0.35,
                    hp: 150,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * RIGHT_WALL, 0.8 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 666.0,
                    shooting_interval : 0.35,
                    hp: 150,
                    ..default()
                }
            ]
            ),
            (5, 1) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.9 * LEFT_WALL, 0.7 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 400.0,
                    shooting_interval : 0.4,
                    hp: 80,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.6 * LEFT_WALL, 0.4 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 400.0,
                    shooting_interval : 0.4,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * LEFT_WALL, 0.2 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.35,
                    hp: 150,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.4 * RIGHT_WALL, 0.6 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.35,
                    hp: 150,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.7 * RIGHT_WALL, 0.9 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.35,
                    hp: 150,
                    ..default()
                }
            ]
            ),
            (5, 2) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * LEFT_WALL, 0.5 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 450.0,
                    shooting_interval : 0.4,
                    hp: 1000,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.4 * LEFT_WALL, 0.1 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.4,
                    hp: 1000,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.1 * RIGHT_WALL, 0.4 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.4,
                    hp: 1000,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.5 * RIGHT_WALL, 0.8 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 450.0,
                    shooting_interval : 0.4,
                    hp: 1000,
                    ..default()
                }
            ]
            ),
            (5, 3) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * LEFT_WALL, 0.2 * LEFT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.35,
                    hp: 2200,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * RIGHT_WALL, 0.8 * RIGHT_WALL) , 
                        Vec2::new(0.2 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.35,
                    hp: 2200,
                    ..default()
                }
            ]
            ),
            (5, 4) => WaveConfig::Duplicate(
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.1 * LEFT_WALL, 0.1 * RIGHT_WALL) , 
                        Vec2::new(0.7 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 666.0,
                    shooting_interval : 0.2,
                    hp: 5000,
                    ..default()
                }, 
                1
            ),
            (5, 5) => WaveConfig::Detailed(vec![
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.8 * RIGHT_WALL, 0.9 * RIGHT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.6 * RIGHT_WALL, 0.7 * RIGHT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.4 * RIGHT_WALL, 0.5 * RIGHT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.2 * RIGHT_WALL, 0.3 * RIGHT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.0 , 0.1 * RIGHT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.1 * LEFT_WALL, 0.0) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.3 * LEFT_WALL, 0.2 * LEFT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.5 * LEFT_WALL, 0.4 * LEFT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.7 * LEFT_WALL, 0.6 * LEFT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
                EnemyConfig {
                    position: PositionConfig::Random(
                        Vec2::new(0.9 * LEFT_WALL, 0.8 * LEFT_WALL) , 
                        Vec2::new(0.5 * TOP_WALL, 0.8 * TOP_WALL)
                    ),
                    bullet_direction: BulletDirectionConfig::Trace,
                    bullet_speed : 500.0,
                    shooting_interval : 0.3,
                    hp: 120,
                    ..default()
                },
            ]  
            ),
            _ => unimplemented!("Level {} wave {} is not implemented", level, wave),
        }
    }
}