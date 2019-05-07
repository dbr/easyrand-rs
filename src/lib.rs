extern crate rand;

use rand::Rng;

/// Returns a random numnber between 0.0 and 1.0
pub fn random() -> f64 {
    rand::random::<f64>()
}

/// Returns a random number in the specified range
pub fn randrange(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min, max)
}

/// Returns a random integer in the specified range
pub fn randint(min: i64, max: i64) -> i64 {
    rand::thread_rng().gen_range(min, max)
}

use rand::seq::SliceRandom;

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

pub fn with_seed(seed: u64) -> SeededRand {
    use rand::SeedableRng;

    SeededRand {
        rng: rand::rngs::StdRng::seed_from_u64(seed),
    }
}

pub struct SeededRand {
    rng: rand::rngs::StdRng,
}

impl SeededRand {
    pub fn random(&mut self) -> f64 {
        self.rng.gen()
    }

    pub fn randrange(&mut self, min: f64, max: f64) -> f64 {
        self.rng.gen_range(min, max)
    }

    pub fn shuffle<T>(obj: &mut Vec<T>) {
        let mut rng = rand::thread_rng();
        obj.shuffle(&mut rng);
    }

    pub fn choose<T>(obj: &Vec<T>) -> Option<&T> {
        let mut rng = rand::thread_rng();
        obj.choose(&mut rng)
    }
}
