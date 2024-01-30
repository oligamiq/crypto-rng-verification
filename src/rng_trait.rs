pub trait RNG {
    fn new() -> Self;
    fn get_random(&mut self) -> u64;
}

pub fn u8_to_u64(buf: &[u8; 8]) -> u64 {
    let mut r: u64 = 0;
    for i in 0..8 {
        r = r << 8;
        r = r | buf[i] as u64;
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

pub fn gen_seed_u8_32() -> [u8; 32] {
    let rng: [u8; 32] = crypto_api_osrandom::to_array().unwrap();
    rng
}
