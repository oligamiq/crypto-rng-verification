use romu::Rng512 as ImplRomu;

use crate::rng_trait::{gen_seed_u64_8, u8_to_u64, RNG};

pub struct Romu {
    romu: ImplRomu,
}

impl Romu {
    pub fn new() -> Self {
        Self {
            romu: ImplRomu::from_seed_with_64bit(gen_seed_u64_8()),
        }
    }
}

impl RNG for Romu {
    fn get_random(&mut self) -> u64 {
        let mut buff = [0u8; 8];
        self.romu.fill_bytes(buff.as_mut());
        u8_to_u64(buff.as_ref())
    }
}

impl Default for Romu {
    fn default() -> Self {
        Self::new()
    }
}
