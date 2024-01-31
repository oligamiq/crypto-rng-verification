use ranluxpp_rs::Ranluxpp as ImplRanluxpp;

use crate::rng_trait::{gen_seed_u64, RNG};

pub struct Ranluxpp {
    ranluxpp: ImplRanluxpp,
}

impl Ranluxpp {
    pub fn new() -> Self {
        Self {
            ranluxpp: ImplRanluxpp::new(gen_seed_u64()),
        }
    }
}

impl RNG for Ranluxpp {
    fn get_random(&mut self) -> u64 {
        let mut buff = [0u64; 9];
        self.ranluxpp.rand(&mut buff);
        buff[0]
    }
}

impl Default for Ranluxpp {
    fn default() -> Self {
        Self::new()
    }
}
