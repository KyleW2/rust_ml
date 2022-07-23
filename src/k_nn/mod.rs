
pub struct Instance {
    points: Vec<f64>,
    label: i32,
}

impl Instance {
    pub fn new(p: Vec<f64>, l: i32) -> Self {
        return Self {
            points:p, 
            label: l
        }
    }
}

impl std::fmt::Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Points: {:?}, Lable: {}", self.points, self.label)
    }
}

pub struct KNN {
    k: i32,
    data: Vec<Instance>,
}

impl KNN {
    pub fn new(k: i32) -> Self {
        return Self {
            k: k,
            data: Vec::new(),
        }
    }

    fn euclidean_distance(x: &Vec<f64>, y: &Vec<f64>) -> f64{
        // Ensure vector length the same
        if x.len() != y.len() {
            return 0.0
        }

        // Sum 
        let mut sum: f64 = 0.0;
        for i in 0..x.len() {
            sum += (y[i] - x[i]).powf(2.0);
        }

        return sum.powf(0.5)
    }

    pub fn set_data(&mut self, data: Vec<Instance>) {
        self.data = data;
    }

    fn calculate_distances(&self, c: &Vec<f64>) -> Vec::<(i32, f64)> {
        // TODO: Make sure self has actual data before calling this fn

        let mut distances = Vec::new();

        for i in 0..self.data.len() {
            distances.push((self.data[i].label, Self::euclidean_distance(&self.data[i].points, c)));
        }

        distances.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
        return distances
    }

    pub fn calculate_distances_threaded(&self, c: &Vec<f64>, threads: usize) -> Vec::<(i32, f64)> {
        let mut distances = Vec::new();

        let step = self.data.len() / threads;

        for i in 0..threads {
            let temp = &self.data[step * i .. step * (i + 1)];
            
            for i in 0..temp.len() {
                println!("{}", temp[i])
            }

            println!()
        }

        return distances
    }

    pub fn classify(&self, c: &Vec<f64>) -> f64 {
        let distances = Self::calculate_distances(&self, &c);

        let mut sum: f64 = 0.0;
        for i in 0..self.k {
            sum += distances[i as usize].0 as f64;
        }

        return sum / self.k as f64
    }
}