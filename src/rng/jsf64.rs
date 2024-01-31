use rand_chacha::rand_core::{RngCore as _, SeedableRng as _};
use small_rngs::Jsf64Rng as ImplJsf64Rng;

use crate::rng_trait::{gen_seed_u8_8, RNG};

pub struct Jsf64Rng {
    jsf64rng: ImplJsf64Rng,
}

impl Jsf64Rng {
    pub fn new() -> Self {
        Self {
            jsf64rng: ImplJsf64Rng::from_seed(gen_seed_u8_8()),
        }
    }
}

impl RNG for Jsf64Rng {
    fn get_random(&mut self) -> u64 {
        self.jsf64rng.next_u64()
    }
}

impl Default for Jsf64Rng {
    fn default() -> Self {
        Self::new()
    }
}
