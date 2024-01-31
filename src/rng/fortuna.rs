use crypto::fortuna::Fortuna as ImplFortuna;
use xorshift::{Rng, SeedableRng};

use crate::rng_trait::{gen_seed_u8_16, RNG};

pub struct Fortuna {
    fortuna: ImplFortuna,
}

impl Fortuna {
    pub fn new() -> Self {
        Self {
            fortuna: ImplFortuna::from_seed(gen_seed_u8_16().as_slice()),
        }
    }
}

impl RNG for Fortuna {
    fn get_random(&mut self) -> u64 {
        self.fortuna.next_u64()
    }
}
impl Default for Fortuna {
    fn default() -> Self {
        Self::new()
    }
}
