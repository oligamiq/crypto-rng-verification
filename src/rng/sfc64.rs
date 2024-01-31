use rand_chacha::rand_core::{RngCore as _, SeedableRng as _};
use small_rngs::Sfc64Rng as ImplSfc64Rng;

use crate::rng_trait::{gen_seed_u8_24, RNG};

pub struct Sfc64Rng {
    sfc64rng: ImplSfc64Rng,
}

impl Sfc64Rng {
    pub fn new() -> Self {
        Self {
            sfc64rng: ImplSfc64Rng::from_seed(gen_seed_u8_24()),
        }
    }
}

impl RNG for Sfc64Rng {
    fn get_random(&mut self) -> u64 {
        self.sfc64rng.next_u64()
    }
}

impl Default for Sfc64Rng {
    fn default() -> Self {
        Self::new()
    }
}
