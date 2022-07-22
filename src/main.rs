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

    // Benchmark
    let now = Instant::now();
    test.classify(&vec![1.2, 1.78]);
    println!("Classified in {} milliseconds", now.elapsed().as_millis());
}