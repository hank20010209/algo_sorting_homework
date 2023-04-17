use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn generate_vector(n: usize) -> Vec<i32> {
    let mut rng = thread_rng();
    let mut vec: Vec<i32> = (1..=n as i32).collect();
    vec.shuffle(&mut rng);
    vec
}