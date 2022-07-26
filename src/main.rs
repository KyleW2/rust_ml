use std::time::Instant;
use rand::Rng;

use rust_ml::knn::KNN;
use rust_ml::instance::Instance;
use rust_ml::loader::{load_instances,write_instances};
use rust_ml::perceptron::Perceptron;

fn main() {
    let mut test = KNN::new(3);
    let mut data: Vec<Instance> = Vec::new();

    let mut rng = rand::thread_rng();

    /* Random data for run time bench */
    for _i in 0..100000 {
        data.push(Instance::new(vec![rng.gen(), rng.gen(), rng.gen(), rng.gen()], rng.gen_range(0..1)))
    }
    
    write_instances(&data, "test_random.inst".to_string());

    let c = vec![rng.gen(), rng.gen(), rng.gen(), rng.gen()];

    let loaded_data = load_instances("test_random.inst".to_string());

    test.set_data(loaded_data);

    // Benchmark single thread
    let now = Instant::now();
    println!("Single threaded KNN classified as label {} in {} milliseconds", test.classify(&c, 1), now.elapsed().as_millis());

    // Benchmark multi thread
    let now = Instant::now();
    println!("Two thread KNN classified as label {} in {} milliseconds", test.classify(&c, 2), now.elapsed().as_millis());

    let mut ptron = Perceptron::new(data, 0.05);
    let its: usize = 10;
    let now = Instant::now();
    ptron.train(its);
    println!("Perceptron did {} weight updates in {} milliseconds", its, now.elapsed().as_millis());
    println!("Perceptron classified as label {}", ptron.classify(&c, false));
}