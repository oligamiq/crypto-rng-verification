use rand_chacha::rand_core::{RngCore as _, SeedableRng as _};
use rand_isaac::Isaac64Rng as ImplIsaacRng;

use crate::rng_trait::{gen_seed_u64, RNG};

pub struct IsaacRng {
    isaacrng: ImplIsaacRng,
}

impl IsaacRng {
    pub fn new() -> Self {
        Self {
            isaacrng: ImplIsaacRng::seed_from_u64(gen_seed_u64()),
        }
    }
}

impl RNG for IsaacRng {
    fn get_random(&mut self) -> u64 {
        self.isaacrng.next_u64()
    }
}
impl Default for IsaacRng {
    fn default() -> Self {
        Self::new()
    }
}
