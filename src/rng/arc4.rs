use std::{marker::PhantomPinned, pin::Pin};

use arc4::Arc4 as ImplArc4;

use crate::rng_trait::{gen_seed_u8_32, u8_to_u64, RNG};

pub struct Arc4<'a> {
    arc4: Option<ImplArc4<'a>>,
    _seed: Pin<Box<SeedWrapper>>,
}

struct SeedWrapper {
    seed: [u8; 32],
    _pin: PhantomPinned,
}

impl SeedWrapper {
    fn new(seed: [u8; 32]) -> Self {
        Self {
            seed,
            _pin: PhantomPinned,
        }
    }

    unsafe fn get_seed_ref(&self) -> *const [u8; 32] {
        &self.seed as *const [u8; 32]
    }
}

impl<'a> RNG for Arc4<'a> {
    fn new() -> Self {
        let seed = gen_seed_u8_32();
        let pinned_seed = Box::pin(SeedWrapper::new(seed));
        let impl_arc4: ImplArc4 = ImplArc4::with_key(unsafe { &*pinned_seed.get_seed_ref() });
        Self {
            arc4: Some(impl_arc4),
            _seed: pinned_seed,
        }
    }
    fn get_random(&mut self) -> u64 {
        let mut buff = [0u8; 8];
        if let Some(ref mut arc4) = self.arc4 {
            arc4.prga(buff.as_mut());
        }
        u8_to_u64(&buff)
    }
}
