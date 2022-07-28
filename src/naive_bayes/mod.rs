use std::collections::HashMap;

pub struct NaiveBayes {
    tokens: HashMap<String, i32>,
    probs: HashMap<String, i32>,
}

impl NaiveBayes {
    pub fn new() -> Self {
        return Self {
            tokens: HashMap<String, i32>::new(),
            probs: HashMap<String, i32>::new(),
        }
    }
}