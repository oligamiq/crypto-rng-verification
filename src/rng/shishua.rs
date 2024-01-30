use shishua_rs::ShiShuARng as ImplShiShuA;

use crate::rng_trait::{gen_seed_u64_4, RNG};

pub struct ShiShuA {
    shishua: ImplShiShuA,
}

impl RNG for ShiShuA {
    fn new() -> Self {
        Self {
            shishua: ImplShiShuA::new(gen_seed_u64_4()),
        }
    }

    fn get_random(&mut self) -> u64 {
        let mut r = 0u64;
        for _ in 0..8 {
            r = r << 8;
            r = r | self.shishua.get_byte() as u64;
        }
        r
    }
}
