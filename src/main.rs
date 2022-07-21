use k_nn::KNN;
use k_nn::Instance;
pub mod k_nn;

fn main() {
    let mut test = KNN::new(3);
    let data: Vec<Instance> = vec![
                                    Instance::new(vec![1.5, 2.0], 1),
                                    Instance::new(vec![3.0, 5.0], 2),
                                    Instance::new(vec![1.3, 1.89], 1)
                                ];
    test.set_data(data);
    println!("{}", test.classify(&vec![1.2, 1.78]));
}
