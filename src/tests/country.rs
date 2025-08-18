#![allow(unused_imports)]
use crate::core::{
    country::{Country, Ideology},
    province::Province,
};
use rand::{SeedableRng, rngs::SmallRng};
#[test]
fn test() {
    let mut rng = SmallRng::seed_from_u64(123_u64);
    let p = Province::new("epl_province".to_string(), &mut rng);
    let c = Country::new("example".to_string(), &mut rng, vec![p]);
    println!("{:#?}", c)
}
