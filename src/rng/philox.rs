use philox::Philox4x32_10 as ImplPhilox;

use crate::rng_trait::{gen_seed_u64, RNG};

pub struct Philox {
    philox: ImplPhilox,
}

impl Philox {
    pub fn new() -> Self {
        Self {
            philox: ImplPhilox::from_key([gen_seed_u64() as u32, gen_seed_u64() as u32].into())
                .set_ctr(Default::default()),
        }
    }
}

impl RNG for Philox {
    fn get_random(&mut self) -> u64 {
        let r: philox::GenericArray<u8, _> = self.philox.next_bytes();
        use std::mem::transmute as tr;
        let r = unsafe {
            [
                tr::<_, u32>(*(&r[0] as *const _ as *const [u8; 4])),
                tr::<_, u32>(*(&r[4] as *const _ as *const [u8; 4])),
                tr::<_, u32>(*(&r[8] as *const _ as *const [u8; 4])),
                tr::<_, u32>(*(&r[12] as *const _ as *const [u8; 4])),
            ]
        };
        r[0] as u64 | ((r[1] as u64) << 32)
    }
}

impl Default for Philox {
    fn default() -> Self {
        Self::new()
    }
}
