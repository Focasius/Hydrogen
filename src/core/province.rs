#![allow(dead_code)]
use rand::{Rng, rngs::SmallRng};
// TODO
#[derive(Debug)]
pub struct Province {
    name: String,
    loyalty: u8,
}

impl Province {
    pub fn new(name: String, rng: &mut SmallRng) -> Self {
        //let mut rng = SmallRng::seed_from_u64(seed);
        Province {
            name,
            loyalty: rng.random(),
        }
    }
}
