use super::instance::Instance;

pub struct Perceptron {
    data: Vec<Instance>,
    weights: Vec<f64>,
    learning_rate: f64,
}

impl Perceptron {
    pub fn new(data: Vec<Instance>, learning_rate: f64) -> Self {
        let mut weights: Vec<f64> = Vec::new();

        for _ in 0..data[0].points.len() {
            weights.push(0.0);
        }

        return Self {
            data: data,
            weights: weights,
            learning_rate: learning_rate,
        }
    }

    fn dot(&self, x: &Vec<f64>) -> f64 {
        if x.len() != self.weights.len() {
            panic!("Instance is not same length as weights")
        }

        let mut sum: f64 = 0.0;
        for i in 0..x.len() {
            sum += self.weights[i] * x[i];
        }

        return sum
    }

    fn sigmoid(x: f64) -> f64 {
        let e: f64 = 2.71828182845904523536028747135266250;
        return 1.0 / (1.0 + e).powf(-1.0 * x)
    }

    fn sign(x: f64) -> f64{
        if x > 0.0 {
            return 1.0
        }
        return -1.0
    }

    fn compute_weight(&mut self, iterations: usize) {
        for _ in 0..iterations {
            for i in 0..self.data.len() {
                let t = f64::from(self.data[i].label);
                // Ignoring adding a bias for now
                let x = &self.data[i].points;
                let o = Self::sign(Self::dot(self, x));

                if o != t {
                    for k in 0..self.weights.len() {
                        self.weights[k] += self.learning_rate * (t - o) * x[k]
                    }
                }
            }
        }
    }

    pub fn train(&mut self, iterations: usize) {
        Self::compute_weight(self, iterations)
    }

    pub fn classify(&self, x: &Vec<f64>, sigmoid: bool) -> f64 {
        if sigmoid {
            return Self::sigmoid(Self::dot(self, &x));
        }
        return Self::sign(Self::dot(self, &x));
    }
}