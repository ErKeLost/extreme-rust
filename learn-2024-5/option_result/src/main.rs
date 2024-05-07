fn main() {
    println!("Hello, world!");

    let a: Option<String> = Some("Hello".to_string());
    println!("{:?}", a.unwrap());

    let line = "1\n2\n3\n4\n";

    for num in line.lines() {
        match num.parse::<i32>().map(|i| i * 2) {
            Ok(n) => println!("{n}"),
            Err(..) => {}
        }
    }

    // 迭代器

    let c = vec![1, 2, 3, 4, 5];
    let mut d = c.into_iter();
    while let Some(n) = d.next() {
        println!("{:?}", n);
    }
}
