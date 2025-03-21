use rand::prelude::ThreadRng;
use rand::Rng;

pub struct RandomChordProgressionComposer {
    rng: ThreadRng,
    last_degree: usize,
}
impl RandomChordProgressionComposer {
    pub fn new(last_degree: usize) -> Self {
        let mut rng = rand::rng();
        Self {
            //
            rng,
            last_degree,
        }
    }
    pub fn generate_new_degree(&mut self) -> usize {
        let mut attempts = 0;
        loop {
            // Random degree. From 1 to 7.
            let degree = self.rng.random_range(1..=7usize);

            if degree != self.last_degree {
                self.last_degree = degree;
                return degree;
            }
            attempts += 1;
            if attempts > MAX_ATTEMPTS_FOR_RANDOM_GENERATION {
                break;
            }
        }

        // Should never happen.
        self.last_degree
    }
}
const MAX_ATTEMPTS_FOR_RANDOM_GENERATION: i32 = 10;
