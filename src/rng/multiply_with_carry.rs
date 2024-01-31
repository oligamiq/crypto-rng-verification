use prandom::multiply_with_carry::MultiplyWithCarry as ImplMultiplyWithCarry;

use crate::rng_trait::{gen_seed_u64, RNG};

pub struct MultiplyWithCarry {
    multiply_with_carry: ImplMultiplyWithCarry,
}
impl MultiplyWithCarry {
    pub fn new() -> Self {
        let mut rng = ImplMultiplyWithCarry::default();
        rng.init_by_seed(gen_seed_u64() as usize);
        Self {
            multiply_with_carry: rng,
        }
    }
}
impl RNG for MultiplyWithCarry {
    fn get_random(&mut self) -> u64 {
        self.multiply_with_carry.take() as u64
    }
}
impl Default for MultiplyWithCarry {
    fn default() -> Self {
        Self::new()
    }
}
