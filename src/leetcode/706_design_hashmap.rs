struct MyHashMap {
    data: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        Self { data: vec![-1; 1000001] }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.data[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.data[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.data[key as usize] = -1
    }
}

pub fn te() {
    let mut m = MyHashMap::new();
    m.put(22, 1);
    println!("{:?}", m.get(22));
    m.remove(22);
    println!("{:?}", m.get(22));
}