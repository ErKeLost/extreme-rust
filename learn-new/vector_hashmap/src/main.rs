use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    init_vec()
    fn init_hashmap() {
        let _: HashMap<String, i64> = HashMap::new();
        let _: HashMap<String, i64> = HashMap::with_capacity(16);
    }
}

fn init_vec() {
    let a: Vec<i32> = Vec::new();
    let a: Vec<i32> = vec![];
    let a: Vec<i32> = vec![1, 2, 3];
    // let a: Vec<i32> = vec![0; 10];
    // let a: Vec<i32> = Vec::with_capacity(10);
    println!("init_vec {:?}", a[0]);
}
