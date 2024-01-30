use aes_prng::AesRng as ImplAesRng;
use rand_chacha::rand_core::{RngCore as _, SeedableRng as _};

use crate::rng_trait::{gen_seed_u8_16, u8_to_u64, RNG};

pub struct AesRng {
    aesrng: ImplAesRng,
}

impl RNG for AesRng {
    fn new() -> Self {
        Self {
            aesrng: ImplAesRng::from_seed(gen_seed_u8_16()),
        }
    }

    fn get_random(&mut self) -> u64 {
        let mut buff = [0u8; 8];
        self.aesrng.fill_bytes(buff.as_mut());
        u8_to_u64(buff.as_ref())
    }
}
