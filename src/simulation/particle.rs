use glam::Vec3;
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Particle {
    pub position: Vec3,
    // TODO: Add color, trail history, etc. as needed
}

impl Particle {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }

    // Create particles with random positions near the attractor
    pub fn spawn_batch(count: usize) -> Vec<Self> {
        let mut rng = rand::thread_rng();

        (0..count).map(|_|{
            Particle::new(Vec3::new(
                rng.gen_range(-1e-2..1e-2),
                rng.gen_range(-1e-2..1e-2),
                rng.gen_range(-1e-2..1e-2)
            ))
        }).collect()
    }
}
