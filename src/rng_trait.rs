use std::ops::Range;

pub trait RNG {
    fn get_random(&mut self) -> u64;

    // fn gen_range(&mut self, min: f64, max: f64) -> f64 {
    //     let diff = max - min;
    //     let r = self.get_random();
    //     let r = r as f64 / std::u64::MAX as f64;
    //     r * diff + min
    // }

    fn gen_range(&mut self, range: Range<f64>) -> f64 {
        let diff = range.end - range.start;
        let r = self.get_random();
        let r = r as f64 / std::u64::MAX as f64;
        r * diff + range.start
    }
}

pub fn u8_to_u64(buf: &[u8]) -> u64 {
    let mut r: u64 = 0;
    for i in 0..8 {
        r = r << 8;
        r = r | buf[i] as u64;
    }
    r
}

// pub fn crypto_random<const LEN: usize>() -> [u8; LEN] {
//     let rng = crypto_api_osrandom::to_array().unwrap();
//     rng
// }

use rand::prelude::*;
// Rust標準で生成する乱数を返す
pub fn crypto_random<const LEN: usize>() -> [u8; LEN] {
    let mut rng = rand::thread_rng();
    let mut rng = (0..LEN).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>();
    let mut r = [0; LEN];
    for i in 0..LEN {
        r[i] = rng.pop().unwrap();
    }
    r
}

pub fn gen_seed_u32() -> u32 {
    let rng: [u8; 4] = crypto_random();
    let mut r: u32 = 0;
    for i in 0..4 {
        r = r << 8;
        r = r | rng[i] as u32;
    }
    r
}

pub fn gen_seed_u64() -> u64 {
    let rng: [u8; 8] = crypto_random();
    u8_to_u64(&rng)
}

pub fn gen_seed_u128() -> u128 {
    let rng: [u8; 16] = crypto_random();
    let mut r: u128 = 0;
    for i in 0..16 {
        r = r << 8;
        r = r | rng[i] as u128;
    }
    r
}

pub fn gen_seed_u8_4() -> [u8; 4] {
    let rng: [u8; 4] = crypto_random();
    rng
}

pub fn gen_seed_u8_8() -> [u8; 8] {
    let rng: [u8; 8] = crypto_random();
    rng
}

pub fn gen_seed_u8_16() -> [u8; 16] {
    let rng: [u8; 16] = crypto_random();
    rng
}

pub fn gen_seed_u8_24() -> [u8; 24] {
    let rng: [u8; 24] = crypto_random();
    rng
}

pub fn gen_seed_u8_32() -> [u8; 32] {
    let rng: [u8; 32] = crypto_random();
    rng
}

pub fn gen_seed_u64_4() -> [u64; 4] {
    let rng: [u8; 32] = crypto_random();
    let mut r: [u64; 4] = [0; 4];
    for i in 0..4 {
        for j in 0..8 {
            r[i] = r[i] << 8;
            r[i] = r[i] | rng[i * 8 + j] as u64;
        }
    }
    r
}

pub fn gen_seed_u64_8() -> [u64; 8] {
    let rng: [u8; 64] = crypto_random();
    let mut r: [u64; 8] = [0; 8];
    for i in 0..8 {
        for j in 0..8 {
            r[i] = r[i] << 8;
            r[i] = r[i] | rng[i * 8 + j] as u64;
        }
    }
    r
}

// pub fn gen_seed_u128_15() -> [randen::U128A; 15] {
//     let rng: [u8; 240] = crypto_random();
//     let mut r: [u128; 15] = [0; 15];
//     for i in 0..15 {
//         for j in 0..16 {
//             r[i] = r[i] << 8;
//             r[i] = r[i] | rng[i * 16 + j] as u128;
//         }
//     }
//     let mut r2 = Vec::new();
//     for i in 0..15 {
//         r2.push(randen::U128A(r[i]));
//     }
//     r2.try_into().unwrap()
// }
