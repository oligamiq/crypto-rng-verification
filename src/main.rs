use ranluxpp_rs::Ranluxpp;
use yarrow_rs::Yarrow;
pub mod rng;
pub mod rng_trait;

fn main() {
    println!("Hello, world!");
    let mut rng = Ranluxpp::new(1234);
    let mut x = [0u64; 9];
    for _ in 0..9 {
        rng.rand(&mut x);
        println!("Random number: {:?}", x);
    }

    let mut yarrow = Yarrow::new(1234);
    let r = yarrow.generate_random_bytes(9);
    println!("Random number: {:?}", r);
}
