use rand::{thread_rng, Rng};



pub fn random_range(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();

    // Exclusive range
    rng.gen_range(min..max)
}