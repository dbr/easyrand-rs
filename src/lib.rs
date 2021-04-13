extern crate rand;

use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;

/// Returns a random numnber between 0.0 and 1.0
pub fn random() -> f64 {
    rand::random::<f64>()
}

/// Returns a random number in the specified range
pub fn randrange(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

/// Returns a random integer in the specified range
pub fn randint(min: i64, max: i64) -> i64 {
    rand::thread_rng().gen_range(min..max)
}

/// In-place shuffle of the given object
pub fn shuffle<T>(obj: &mut Vec<T>) {
    let mut rng = rand::thread_rng();
    obj.shuffle(&mut rng);
}

/// Return a reference to a randomly selected item in the given object
pub fn choose<T>(obj: &Vec<T>) -> Option<&T> {
    let mut rng = rand::thread_rng();
    obj.choose(&mut rng)
}

/// Convinence method to get seedable
pub fn with_seed(seed: u64) -> SeededRand {
    SeededRand::new(seed)
}

pub struct SeededRand {
    rng: rand::rngs::StdRng,
}

/// Creates predictible random numbers using a given seed value
impl SeededRand {
    pub fn new(seed: u64) -> Self {
        SeededRand {
            rng: rand::rngs::StdRng::seed_from_u64(seed),
        }
    }

    /// Returns a random numnber between 0.0 and 1.0
    pub fn random(&mut self) -> f64 {
        self.rng.gen()
    }

    /// Returns a random number in the specified range
    pub fn randrange(&mut self, min: f64, max: f64) -> f64 {
        self.rng.gen_range(min..max)
    }

    /// In-place shuffle of the given object
    pub fn shuffle<T>(obj: &mut Vec<T>) {
        let mut rng = rand::thread_rng();
        obj.shuffle(&mut rng);
    }

    /// Return a reference to a randomly selected item in the given object
    pub fn choose<T>(obj: &Vec<T>) -> Option<&T> {
        let mut rng = rand::thread_rng();
        obj.choose(&mut rng)
    }
}
