fn main() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(5);
    v.push(5);
    v.push(5);

    println!("a = {:?}", v);

    let mut v2 = vec![1, 2, 3, 4, 5];
    // println!("v2 = {:?}", v2[99]);
    match v.get(99) {
        Some(x) => println!("v2 = {:?}", x),
        None => println!("v2 = None"),
    }

    for i in &mut v2 {
        *i += 50;
    }
    println!("v2 = {:?}", v2);

    let m1 = String::from("hello ");
    let m2 = String::from("world");
    println!("{}", format!("{}{}", m1, m2));
}
