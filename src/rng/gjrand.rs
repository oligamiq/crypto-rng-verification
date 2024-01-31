use rand_chacha::rand_core::{RngCore as _, SeedableRng as _};
use small_rngs::GjRng as ImplGjRng;

use crate::rng_trait::{gen_seed_u8_16, RNG};

pub struct GjRng {
    gjrng: ImplGjRng,
}

impl GjRng {
    pub fn new() -> Self {
        Self {
            gjrng: ImplGjRng::from_seed(gen_seed_u8_16()),
        }
    }
}

impl RNG for GjRng {
    fn get_random(&mut self) -> u64 {
        self.gjrng.next_u64()
    }
}
impl Default for GjRng {
    fn default() -> Self {
        Self::new()
    }
}
