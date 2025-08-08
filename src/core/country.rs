#![allow(dead_code)]

use rand::{self, Rng, SeedableRng, rngs::SmallRng};

#[derive(Debug)]
pub enum Ideology {
    Socialism,
    Communism,
    Capitalism,
}

impl Ideology {
    pub fn random(seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        match rng.random_range(0..2) {
            0 => Ideology::Capitalism,
            1 => Ideology::Communism,
            2 => Ideology::Socialism,
            _ => panic!(r#"Unknown Numbers, Which Cannot Generate An Enumeration "Ideology". "#),
        }
    }
}
// TODO
#[derive(Debug)]
pub struct Country {
    name: String,
    ideology: Ideology,
    population: i128,
    coin: i128,
    stability: u8,
}

impl Country {
    pub fn new(name: String, seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        Country {
            name,
            ideology: Ideology::random(seed),
            population: rng.random_range(0..i128::MAX),
            coin: rng.random(),
            stability: rng.random(),
        }
    }
}
