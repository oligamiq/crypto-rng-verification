use r30_rs::R30 as ImplR30;

use crate::rng_trait::{gen_seed_u32, RNG};

pub struct R30 {
    r30: ImplR30,
}

impl R30 {
    pub fn new() -> Self {
        Self {
            r30: ImplR30::new(gen_seed_u32()),
        }
    }
}

impl RNG for R30 {
    fn get_random(&mut self) -> u64 {
        self.r30.next_u64()
    }
}

impl Default for R30 {
    fn default() -> Self {
        Self::new()
    }
}
