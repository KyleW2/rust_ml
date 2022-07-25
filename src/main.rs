use std::time::Instant;

use k_nn::KNN;
use k_nn::Instance;
use rand::Rng;
pub mod k_nn;

fn main() {
    let mut test = KNN::new(3);
    let mut data: Vec<Instance> = Vec::new();

    let mut rng = rand::thread_rng();

    for _i in 0..100000 {
        data.push(Instance::new(vec![rng.gen(), rng.gen()], rng.gen()))
    }
    test.set_data(data);

    test.calculate_distances_threaded(&vec![rng.gen(), rng.gen()], 5);

    // Benchmark single thread
    let now = Instant::now();
    test.classify(&vec![rng.gen(), rng.gen()], 1);
    println!("Single thread classified in {} milliseconds", now.elapsed().as_millis());

    // Benchmark multi thread
    let now = Instant::now();
    test.classify(&vec![rng.gen(), rng.gen()], 2);
    println!("Two threads classified in {} milliseconds", now.elapsed().as_millis());
}