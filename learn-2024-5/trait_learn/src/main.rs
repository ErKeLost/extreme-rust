struct Point<T> {
    x: T,
    y: T,
}
fn print<T: std::fmt::Display>(p: Point<T>) {
    println!("Point {}, {}", p.x, p.y);
}

fn main() {
    let p = Point { x: 10, y: 20 };
    print(p);
    let p = Point { x: 10.2, y: 20.4 };
    print(p);
}
