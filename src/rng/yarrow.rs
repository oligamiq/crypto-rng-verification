use yarrow_rs::Yarrow as ImplYarrow;

use crate::rng_trait::{gen_seed_u128, u8_to_u64, RNG};

pub struct Yarrow {
    yarrow: ImplYarrow,
}

impl Yarrow {
    pub fn new() -> Self {
        Self {
            yarrow: ImplYarrow::new(gen_seed_u128()),
        }
    }
}

impl RNG for Yarrow {
    fn get_random(&mut self) -> u64 {
        u8_to_u64(&self.yarrow.generate_random_bytes(8))
    }
}

impl Default for Yarrow {
    fn default() -> Self {
        Self::new()
    }
}
