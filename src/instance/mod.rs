pub struct Instance {
    pub points: Vec<f64>,
    pub label: i32,
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