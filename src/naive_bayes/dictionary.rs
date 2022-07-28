pub struct KeyValue<K, V> {
    key: K,
    value: V,
}

pub struct Dictionary<K, V> {
    pairs: Vec<KeyValue<K, V>>,
}

impl<K, V> Dictionary<K, V> {
    pub fn new() -> Self {
        return Self {
            pairs: Vec::new()
        }
    }
}

impl<K, V> std::ops::Index<usize> for Dictionary<K, V> {
    type Output = KeyValue<K, V>;

    fn index(&self, idx: usize) -> &KeyValue<K, V> {
        return &self.pairs[idx]
    }
}

impl<K, V> std::ops::IndexMut   <usize> for Dictionary<K, V> {
    type Output = KeyValue<K, V>;
    
    fn index_mut(&mut self, idx: usize) -> &mut KeyValue<K, V> {
        return &mut self.pairs[idx]
    }
}

fn main() {
    let mut test: Dictionary<String, i32> = Dictionary::new();

}