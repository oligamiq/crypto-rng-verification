use rand_chacha::{
    rand_core::{RngCore as _, SeedableRng as _},
    ChaCha20Rng as ImplChaCha20,
};

use crate::rng_trait::{gen_seed_u8_32, u8_to_u64, RNG};

pub struct ChaCha20 {
    chacha20: ImplChaCha20,
}

impl ChaCha20 {
    pub fn new() -> Self {
        Self {
            chacha20: ImplChaCha20::from_seed(gen_seed_u8_32()),
        }
    }
}

impl RNG for ChaCha20 {
    fn get_random(&mut self) -> u64 {
        let mut buff = [0u8; 8];
        self.chacha20.fill_bytes(buff.as_mut());
        u8_to_u64(&buff)
    }
}
impl Default for ChaCha20 {
    fn default() -> Self {
        Self::new()
    }
}
