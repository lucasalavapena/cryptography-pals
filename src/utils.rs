use rand::{distributions::Uniform, Rng};

pub fn generate_random_vec(key_size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..key_size).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>()
}

pub fn generate_rand_vec_rand_size(lower_bound_size: usize, upper_bound_size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let size_distribution: Uniform<usize> = Uniform::new(lower_bound_size, upper_bound_size);
    let size = rng.sample(size_distribution);

    (0..size).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>()
}
