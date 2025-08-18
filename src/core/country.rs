#![allow(dead_code)]

use rand::{self, Rng, rngs::SmallRng};

use crate::core::province::Province;

#[derive(Debug)]
pub enum Ideology {
    Socialism,
    Communism,
    Capitalism,
}

impl Ideology {
    pub fn random(rng: &mut SmallRng) -> Self {
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
    provinces: Vec<Province>,
}

impl Country {
    pub fn new(name: String, rng: &mut SmallRng, provinces: Vec<Province>) -> Self {
        Country {
            name,
            ideology: Ideology::random(rng),
            population: rng.random_range(0..i128::MAX),
            coin: rng.random(),
            stability: rng.random(),
            provinces,
        }
    }
}
