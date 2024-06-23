//! Deprecated(May be removed in future commits)
//! This file implements the particles in game.

#[derive(Clone, Copy)]
struct ParticleConfig {
    position: Vec2

}

#[derive(Bundle)]
struct ParticleBundle {
    particle_config: ParticleConfig,
    particle_timer: Timer
}

struct ExplosionParticle(Particle);

