use randen::RandenRng as ImplRandenRng;
use xorshift::Rng as _;

use crate::rng_trait::RNG;

pub struct RandenRng {
    randenrng: ImplRandenRng,
}

impl RandenRng {
    pub fn new() -> Self {
        Self {
            randenrng: ImplRandenRng::new_unseeded(),
        }
    }
}

impl RNG for RandenRng {
    fn get_random(&mut self) -> u64 {
        self.randenrng.next_u64()
    }
}

impl Default for RandenRng {
    fn default() -> Self {
        Self::new()
    }
}
