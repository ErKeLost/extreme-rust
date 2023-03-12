fn add_i8(a: i8, b: i8) -> i8 {
    a + b
}

fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}

/**
* 在编程的时候，我们经常利用多态。通俗的讲，多态就是好比坦克的炮管，既可以发射普通弹药，也可以发射制导炮弹（导弹），也可以发射贫铀穿甲弹，甚至发射子母弹，没有必要为每一种炮弹都在坦克上分别安装一个专用炮管，即使生产商愿意，炮手也不愿意，累死人啊。所以在编程开发中，我们也需要这样“通用的炮管”，这个“通用的炮管”就是多态。

   实际上，泛型就是一种多态。泛型主要目的是为程序员提供编程的便利，减少代码的臃肿，同时可以极大地丰富语言本身的表达能力，为程序员提供了一个合适的炮管。想想，一个函数，可以代替几十个，甚至数百个函数，是一件多么让人兴奋的事情：
*
*/

fn add<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
    a * b
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// 结构体中使用泛型定义
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 如果想让 x 和 y 既能类型相同，又能类型不同，就需要使用不同的泛型参数：
struct Point2<T, U> {
    x: T,
    y: U,
}

// 在枚举中使用泛型
enum Option<T> {
    Some(T),
    None,
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // println!("The largest number is {}", result);
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?}", float);
    // 必须要使用相同的类型 不然会报错
    // let error = Point { x: 1.0, y: 4 };

    println!("add generics: {}", add(6, 200));
    println!("add generics: {}", add(1.5, 5.1));
    println!("add generics: {}", add(800, 200));
    println!("add i8: {}", add_i8(1, 2));
    println!("add i8: {}", add_i8(20i8, 3i8));
    println!("add i32: {}", add_i32(800, 600));
    println!("add i32: {}", add_i32(800i32, 600i32));
    println!("add f64: {}", add_f64(1.2, 3.4));
}
