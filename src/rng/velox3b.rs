use rand::Rng as _;
use rand_chacha::rand_core::SeedableRng as _;
use small_rngs::Velox3bRng as ImplVelox3bRng;

use crate::rng_trait::RNG;

pub struct Velox3bRng {
    velox3brng: ImplVelox3bRng,
}

impl Velox3bRng {
    pub fn new() -> Self {
        Self {
            // velox3brng: ImplVelox3bRng::from_seed(gen_seed_u8_4()),
            velox3brng: ImplVelox3bRng::from_rng(rand::thread_rng()).unwrap(),
        }
    }
}

impl RNG for Velox3bRng {
    fn get_random(&mut self) -> u64 {
        self.velox3brng.gen()
    }
}

impl Default for Velox3bRng {
    fn default() -> Self {
        Self::new()
    }
}
