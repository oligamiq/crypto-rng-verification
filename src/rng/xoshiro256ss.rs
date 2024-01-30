use rand_chacha::rand_core::{RngCore as _, SeedableRng as _};
use rand_xoshiro::Xoshiro512StarStar as ImplXoshiro512StarStar;

use crate::rng_trait::{gen_seed_u64, RNG};

pub struct Xoshiro512StarStar {
    xoshiro512starstar: ImplXoshiro512StarStar,
}

impl RNG for Xoshiro512StarStar {
    fn new() -> Self {
        Self {
            xoshiro512starstar: ImplXoshiro512StarStar::seed_from_u64(gen_seed_u64()),
        }
    }

    fn get_random(&mut self) -> u64 {
        self.xoshiro512starstar.next_u64()
    }
}
