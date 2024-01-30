use arc4::Arc4 as ImplArc4;

use crate::rng_trait::{gen_seed_u8_32, u8_to_u64, RNG};

pub struct Arc4 {
  arc4: ImplArc4<'static>,
  _seed: [u8; 32],
}

impl RNG for Arc4 {
  fn new() -> Self {
    let seed = gen_seed_u8_32();
    let seed_p = &seed as *const [u8; 32];
    let arc4: ImplArc4 = ImplArc4::with_key(unsafe { &*seed_p });
    Self {
      arc4,
      _seed: seed,
    }
  }
  fn get_random(&mut self) -> u64 {
      let mut buff = [0u8; 8];
      self.arc4.prga(&mut buff);
      u8_to_u64(&buff)
  }
}
