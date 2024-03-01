use rand::Rng;
use std::time::Instant;

fn sum_array(input_array: &Vec<f64>, _threads: usize) -> f64 {
    input_array.iter().sum()
}

fn generate_array(size: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}

fn main() {
    let array = generate_array(10_000);
    let threads = 1;

    let start = Instant::now();
    let sum = sum_array(&array, threads);
    let duration = start.elapsed();

    println!(
        "Total array size: {}, added in {:?} via {} thread(s)",
        sum, duration, threads
    );
}
