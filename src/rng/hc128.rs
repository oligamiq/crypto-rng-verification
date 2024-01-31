use rand_chacha::rand_core::{RngCore as _, SeedableRng as _};
use rand_hc::Hc128Rng as ImplHc128Rng;

use crate::rng_trait::{gen_seed_u64, RNG};

pub struct Hc128Rng {
    hc128rng: ImplHc128Rng,
}

impl Hc128Rng {
    pub fn new() -> Self {
        Self {
            hc128rng: ImplHc128Rng::seed_from_u64(gen_seed_u64()),
        }
    }
}
impl RNG for Hc128Rng {
    fn get_random(&mut self) -> u64 {
        self.hc128rng.next_u64()
    }
}
impl Default for Hc128Rng {
    fn default() -> Self {
        Self::new()
    }
}
