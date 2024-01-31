use mersenne_twister_m::MT19937 as ImplMT19937;

use crate::rng_trait::{gen_seed_u32, RNG};

pub struct MT19937 {
    mt19937: ImplMT19937,
}

impl MT19937 {
    pub fn new() -> Self {
        Self {
            mt19937: ImplMT19937::new_with_seed(gen_seed_u32()),
        }
    }
}

impl RNG for MT19937 {
    fn get_random(&mut self) -> u64 {
        let a = self.mt19937.genrand() as u64;
        let b = self.mt19937.genrand() as u64;
        (a << 32) | b
    }
}
impl Default for MT19937 {
    fn default() -> Self {
        Self::new()
    }
}
