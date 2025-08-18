#![allow(unused_imports)]
use rand::{SeedableRng, rngs::SmallRng};

use crate::core::province::Province;
#[test]
fn test() {
    let mut rng = SmallRng::seed_from_u64(123_u64);
    let p = Province::new("example".to_string(), &mut rng);
    println!("{:#?}", p);
}
