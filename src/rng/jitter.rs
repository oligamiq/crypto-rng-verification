use rand_chacha::rand_core::RngCore as _;
use rand_jitter::JitterRng as ImplJitterRng;

use crate::rng_trait::RNG;

pub struct JitterRng {
    jitterrng: ImplJitterRng<Box<dyn Fn() -> u64 + Sync + Send>>,
}

fn get_nstime() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let dur = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // The correct way to calculate the current time is
    // `dur.as_secs() * 1_000_000_000 + dur.subsec_nanos() as u64`
    // But this is faster, and the difference in terms of entropy is
    // negligible (log2(10^9) == 29.9).
    dur.as_secs() << 30 | dur.subsec_nanos() as u64
}

impl JitterRng {
    pub fn new() -> Self {
        let f: Box<dyn Fn() -> u64 + Send + Sync> = Box::new(get_nstime);
        let rng = ImplJitterRng::new_with_timer(f);
        Self { jitterrng: rng }
    }
}

impl RNG for JitterRng {
    fn get_random(&mut self) -> u64 {
        self.jitterrng.next_u64()
    }
}

impl Default for JitterRng {
    fn default() -> Self {
        Self::new()
    }
}
