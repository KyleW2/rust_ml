use std::thread;

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

impl Clone for Instance {
    fn clone(&self) -> Instance {
        return Instance { 
            points: self.points.clone(), 
            label: self.label.clone() 
        }
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

    pub fn set_data(&mut self, data: Vec<Instance>) {
        self.data = data;
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
        let mut thread_handles = Vec::new();

        let step = self.data.len() / threads;

        // Segment data into parts
        for i in 0..threads {
            let temp = self.data[step * i .. step * (i + 1)].to_vec().clone();
            let c_to_be_eaten = c.clone();
            
            // Spawn new thread and pass vars
            thread_handles.push(thread::spawn(move || Self::calculate_distance_thread(c_to_be_eaten, temp)))
        }

        // Append thread results to distances
        while thread_handles.len() > 0 {
            // Remove join handle from vec into current thread
            let temp = thread_handles.remove(0);
            // Join and unwrap result
            let mut temp_join = temp.join().unwrap();
            distances.append(&mut temp_join);
        }

        distances.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
        return distances
    }

    fn calculate_distance_thread(c: Vec<f64>, data: Vec<Instance>) -> Vec::<(i32, f64)> {
        let mut distances: Vec<(i32, f64)> = Vec::new();

        for i in 0..data.len() {
            distances.push((data[i].label, Self::euclidean_distance(&data[i].points, &c)))
        }

        return distances
    }

    pub fn classify(&self, c: &Vec<f64>, threads: usize) -> f64 {
        let distances: Vec<(i32, f64)>;

        // For single threaded
        if threads > 1 {
            distances = Self::calculate_distances(&self, &c);
        } 
        // For multi-threaded
        else {
            distances = Self::calculate_distances_threaded(&self, &c, threads);
        }

        let mut sum: f64 = 0.0;

        for i in 0..self.k {
            sum += distances[i as usize].0 as f64;
        }

        return sum / self.k as f64
    }
}