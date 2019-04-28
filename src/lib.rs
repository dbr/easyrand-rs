extern crate rand;

use rand::Rng;

/// Generate a random numnber between 0.0 and 1.0
pub fn random() -> f64 {
    rand::random::<f64>()
}

/// Generates a random number in the specified range
pub fn randrange(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min, max)
}

pub fn randint(min: i64, max: i64) -> i64 {
    rand::thread_rng().gen_range(min, max)
}

use rand::seq::SliceRandom;

pub fn shuffle<T>(obj: &mut Vec<T>) {
    let mut rng = rand::thread_rng();
    obj.shuffle(&mut rng);
}

pub fn choose<T>(obj: &Vec<T>) -> Option<&T> {
    let mut rng = rand::thread_rng();
    obj.choose(&mut rng)
}

pub fn with_seed(seed: u64) -> SeededRand {
    SeededRand { seed: seed }
}

pub struct SeededRand {
    seed: u64,
}

impl SeededRand {
    pub fn random() -> f64 {
        0.0
    }

    pub fn randrange(min: f64, max: f64) -> f64 {
        2.0
    }

    pub fn shuffle() {}

    pub fn choice() {}
}
