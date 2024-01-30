pub trait RNG {
    fn new() -> Self;
    fn get_random(&mut self) -> u64;
}

pub fn u8_to_u64(buf: &[u8]) -> u64 {
    let mut r: u64 = 0;
    for i in 0..8 {
        r = r << 8;
        r = r | buf[i] as u64;
    }
    r
}

pub fn gen_seed_u32() -> u32 {
    let rng: [u8; 4] = crypto_api_osrandom::to_array().unwrap();
    let mut r: u32 = 0;
    for i in 0..4 {
        r = r << 8;
        r = r | rng[i] as u32;
    }
    r
}

pub fn gen_seed_u64() -> u64 {
    let rng: [u8; 8] = crypto_api_osrandom::to_array().unwrap();
    u8_to_u64(&rng)
}

pub fn gen_seed_u128() -> u128 {
    let rng: [u8; 16] = crypto_api_osrandom::to_array().unwrap();
    let mut r: u128 = 0;
    for i in 0..16 {
        r = r << 8;
        r = r | rng[i] as u128;
    }
    r
}

pub fn gen_seed_u8_16() -> [u8; 16] {
    let rng: [u8; 16] = crypto_api_osrandom::to_array().unwrap();
    rng
}

pub fn gen_seed_u8_32() -> [u8; 32] {
    let rng: [u8; 32] = crypto_api_osrandom::to_array().unwrap();
    rng
}

pub fn gen_seed_u64_4() -> [u64; 4] {
    let rng: [u8; 32] = crypto_api_osrandom::to_array().unwrap();
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
    let rng: [u8; 64] = crypto_api_osrandom::to_array().unwrap();
    let mut r: [u64; 8] = [0; 8];
    for i in 0..8 {
        for j in 0..8 {
            r[i] = r[i] << 8;
            r[i] = r[i] | rng[i * 8 + j] as u64;
        }
    }
    r
}
