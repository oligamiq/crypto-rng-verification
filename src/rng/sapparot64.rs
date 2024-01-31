use rand_chacha::rand_core::{RngCore as _, SeedableRng as _};
use small_rngs::Sapparot64Rng as ImplSapparot64Rng;

use crate::rng_trait::{gen_seed_u8_24, RNG};

pub struct Sapparot64Rng {
    sapparot64rng: ImplSapparot64Rng,
}

impl Sapparot64Rng {
    pub fn new() -> Self {
        Self {
            sapparot64rng: ImplSapparot64Rng::from_seed(gen_seed_u8_24()),
        }
    }
}

impl RNG for Sapparot64Rng {
    fn get_random(&mut self) -> u64 {
        self.sapparot64rng.next_u64()
    }
}

impl Default for Sapparot64Rng {
    fn default() -> Self {
        Self::new()
    }
}
