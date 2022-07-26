use std::time::Instant;
use rand::Rng;

use rust_ml::knn::KNN;
use rust_ml::instance::Instance;
use rust_ml::loader::{load_instances,write_instances};

fn main() {
    let mut test = KNN::new(3);
    let mut data: Vec<Instance> = Vec::new();

    let mut rng = rand::thread_rng();

    /* Random data for run time bench */
    for _i in 0..100000 {
        data.push(Instance::new(vec![rng.gen(), rng.gen()], rng.gen()))
    }
    
    write_instances(&data, "test_random.inst".to_string());

    let c = vec![0.5, 0.5];

    test.set_data(load_instances("test_random.inst".to_string()));

    // Benchmark single thread
    let now = Instant::now();
    println!("Single thread classified as label {} in {} milliseconds", test.classify(&c, 1), now.elapsed().as_millis());

    // Benchmark multi thread
    let now = Instant::now();
    println!("Two threads classified as label {} in {} milliseconds", test.classify(&c, 2), now.elapsed().as_millis());
}