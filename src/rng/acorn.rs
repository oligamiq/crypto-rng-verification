use acorn_prng::{Acorn as ImplAcorn, Order, Seed};

use crate::rng_trait::{gen_seed_u128, gen_seed_u64, RNG};

pub struct Acorn {
    acorn: ImplAcorn,
}

impl Acorn {
    pub fn new() -> Self {
        Self {
            acorn: ImplAcorn::new(
                Order::new(gen_seed_u64() as usize),
                Seed::new(gen_seed_u128()),
            ),
        }
    }
}

impl RNG for Acorn {
    fn get_random(&mut self) -> u64 {
        self.acorn.generate_fixed_length_u64(1)
    }
}
impl Default for Acorn {
    fn default() -> Self {
        Self::new()
    }
}
