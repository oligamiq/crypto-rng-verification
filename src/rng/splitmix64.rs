use xorshift::{splitmix64::SplitMix64 as ImplSplitMix64, Rng as _, SeedableRng};

use crate::rng_trait::{gen_seed_u64, RNG};

pub struct SplitMix64 {
    splitmix64: ImplSplitMix64,
}

impl SplitMix64 {
    pub fn new() -> Self {
        Self {
            splitmix64: ImplSplitMix64::from_seed(gen_seed_u64()),
        }
    }
}

impl RNG for SplitMix64 {
    fn get_random(&mut self) -> u64 {
        self.splitmix64.next_u64()
    }
}

impl Default for SplitMix64 {
    fn default() -> Self {
        Self::new()
    }
}
