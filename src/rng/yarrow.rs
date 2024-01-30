use yarrow_rs::Yarrow as ImplYarrow;

use crate::rng_trait::{gen_seed_u128, u8_to_u64, RNG};

pub struct Yarrow {
    yarrow: ImplYarrow,
}

impl RNG for Yarrow {
    fn new() -> Self {
        Self {
            yarrow: ImplYarrow::new(gen_seed_u128()),
        }
    }

    fn get_random(&mut self) -> u64 {
      u8_to_u64(&self.yarrow.generate_random_bytes(8))
    }
}
