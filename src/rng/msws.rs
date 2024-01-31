use rand_chacha::rand_core::{RngCore as _, SeedableRng as _};
use small_rngs::MswsRng as ImplMswsRng;

use crate::rng_trait::RNG;

pub struct MswsRng {
    mswsrng: ImplMswsRng,
}

impl MswsRng {
    pub fn new() -> Self {
        Self {
            // mswsrng: ImplMswsRng::from_seed(gen_seed_u8_16()),
            mswsrng: ImplMswsRng::from_rng(rand::thread_rng()).unwrap(),
        }
    }
}

impl RNG for MswsRng {
    fn get_random(&mut self) -> u64 {
        self.mswsrng.next_u64()
    }
}

impl Default for MswsRng {
    fn default() -> Self {
        Self::new()
    }
}
