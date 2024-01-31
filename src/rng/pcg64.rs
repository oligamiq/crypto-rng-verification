use nanorand::{rand::Pcg64 as ImplPcg64, Rng};

use crate::rng_trait::{gen_seed_u128, u8_to_u64, RNG};

pub struct Pcg64 {
    pcg64: ImplPcg64,
}

impl Pcg64 {
    pub fn new() -> Self {
        Self {
            pcg64: ImplPcg64::new_seed(gen_seed_u128()),
        }
    }
}

impl RNG for Pcg64 {
    fn get_random(&mut self) -> u64 {
        u8_to_u64(&self.pcg64.rand())
    }
}
impl Default for Pcg64 {
    fn default() -> Self {
        Self::new()
    }
}
