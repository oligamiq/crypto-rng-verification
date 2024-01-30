use rand_chacha::ChaCha20Rng as ImplChaCha20;

use crate::rng_trait::RNG;

pub struct ChaCha20 {
    chacha20: ImplChaCha20
}

impl RNG for ChaCha20 {
    fn new() -> Self {
      
    }
}
