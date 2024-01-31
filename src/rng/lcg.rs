use rand_chacha::rand_core::{RngCore, SeedableRng as _};
use small_rngs::PcgXsl64LcgRng as ImplPcgXsl64LcgRng;

use crate::rng_trait::{gen_seed_u8_16, RNG};

pub struct PcgXsl64LcgRng {
    pcgxsl64lcgrng: ImplPcgXsl64LcgRng,
}

impl PcgXsl64LcgRng {
    pub fn new() -> Self {
        Self {
            pcgxsl64lcgrng: ImplPcgXsl64LcgRng::from_seed(gen_seed_u8_16()),
        }
    }
}

impl RNG for PcgXsl64LcgRng {
    fn get_random(&mut self) -> u64 {
        self.pcgxsl64lcgrng.next_u64()
    }
}

impl Default for PcgXsl64LcgRng {
    fn default() -> Self {
        Self::new()
    }
}
