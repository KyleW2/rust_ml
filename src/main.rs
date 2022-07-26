use std::time::Instant;
use rand::Rng;

use rust_ml::knn::KNN;
use rust_ml::instance::Instance;
use rust_ml::loader::load;

fn main() {
    let mut test = KNN::new(3);
    let mut data: Vec<Instance> = Vec::new();

    let mut rng = rand::thread_rng();

    /* Random data for run time bench */
    for _i in 0..100000 {
        data.push(Instance::new(vec![rng.gen(), rng.gen()], rng.gen()))
    }

    /* Specific data for correctness bench
    data.push(Instance::new(vec![0.0, 1.0], 1));
    data.push(Instance::new(vec![1.0, 1.0], 1));
    data.push(Instance::new(vec![0.5, 1.5], 1));
    data.push(Instance::new(vec![-4.0, -2.0], 2));
    data.push(Instance::new(vec![-2.5, -2.8], 2));
    data.push(Instance::new(vec![-4.0, -3.0], 2));
    */
    

    let c = vec![0.5, 0.5];

    test.set_data(data);

    // Benchmark single thread
    let now = Instant::now();
    println!("{}", test.classify(&c, 1));
    println!("Single thread classified in {} milliseconds", now.elapsed().as_millis());

    // Benchmark multi thread
    let now = Instant::now();
    println!("{}", test.classify(&c, 2));
    println!("Two threads classified in {} milliseconds", now.elapsed().as_millis());
}