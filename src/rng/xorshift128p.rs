use xorshift::{Rng as _, SeedableRng, Xorshift128 as ImplXorshift128};

use crate::rng_trait::{gen_seed_u64_4, RNG};

pub struct Xorshift128 {
    xorshift128: ImplXorshift128,
}

impl Xorshift128 {
    pub fn new() -> Self {
        Self {
            xorshift128: SeedableRng::from_seed(gen_seed_u64_4().as_slice()),
        }
    }
}

impl RNG for Xorshift128 {
    fn get_random(&mut self) -> u64 {
        self.xorshift128.next_u64()
    }
}

impl Default for Xorshift128 {
    fn default() -> Self {
        Self::new()
    }
}
