pub mod monte_carlo_integration;
use std::sync::{Arc, Mutex};

use monte_carlo_integration::MonteCarloIntegration as MCI;
pub use monte_carlo_integration::*;
pub mod rng;
use rayon::prelude::*;
pub use rng::*;

pub mod rng_trait;

fn main() {
    let mut mci = Vec::new();
    mci.push(Arc::new(Mutex::new(MCI::template_new(Acorn::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(AesRng::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Arc4::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(ChaCha20::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Fortuna::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(GjRng::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Hc128Rng::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(IsaacRng::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(JitterRng::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Jsf64Rng::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(
        PcgXsl64LcgRng::new(),
    ))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Lehmer::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(MT19937::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(MswsRng::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(
        MultiplyWithCarry::new(),
    ))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Pcg64::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Philox::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(RandenRng::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Ranluxpp::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Romu::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(R30::new()))));
    mci.push(Arc::new(Mutex::new(
        MCI::template_new(Sapparot64Rng::new()),
    )));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Sfc64Rng::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(ShiShuA::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(SplitMix64::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Velox3bRng::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(WyRand::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Xorshift128::new()))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(
        Xoshiro512StarStar::new(),
    ))));
    mci.push(Arc::new(Mutex::new(MCI::template_new(Yarrow::new()))));
    let err = mci
        .par_iter()
        .map(|mc| {
            let mut mc = mc.lock().unwrap();
            mc.err(1000000)
        })
        .collect::<Vec<f64>>();
    println!("err: {:?}", err);
}
