use nanorand::{rand::WyRand as ImplWyRand, Rng};

use crate::rng_trait::{gen_seed_u64, RNG};

pub struct WyRand {
    wyrand: ImplWyRand,
}

impl WyRand {
    pub fn new() -> Self {
        Self {
            wyrand: ImplWyRand::new_seed(gen_seed_u64()),
        }
    }
}

impl RNG for WyRand {
    fn get_random(&mut self) -> u64 {
        self.wyrand.generate()
    }
}

impl Default for WyRand {
    fn default() -> Self {
        Self::new()
    }
}
