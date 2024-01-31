use prng::PRNG as ImplLehmer;

use crate::rng_trait::{gen_seed_u64, RNG};

pub struct Lehmer {
    lehmer: ImplLehmer,
}
impl Lehmer {
    pub fn new() -> Self {
        Self {
            lehmer: ImplLehmer::new(gen_seed_u64()),
        }
    }
}

impl RNG for Lehmer {
    fn get_random(&mut self) -> u64 {
        self.lehmer.next_unsigned_integer()
    }
}
impl Default for Lehmer {
    fn default() -> Self {
        Self::new()
    }
}
