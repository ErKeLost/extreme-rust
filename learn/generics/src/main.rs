// fn add_i8(a: i8, b: i8) -> i8 {
//     a + b
// }

// fn add_i32(a: i32, b: i32) -> i32 {
//     a + b
// }

// fn add_f64(a: f64, b: f64) -> f64 {
//     a + b
// }

// /**
// * 在编程的时候，我们经常利用多态。通俗的讲，多态就是好比坦克的炮管，既可以发射普通弹药，也可以发射制导炮弹（导弹），也可以发射贫铀穿甲弹，甚至发射子母弹，没有必要为每一种炮弹都在坦克上分别安装一个专用炮管，即使生产商愿意，炮手也不愿意，累死人啊。所以在编程开发中，我们也需要这样“通用的炮管”，这个“通用的炮管”就是多态。

//    实际上，泛型就是一种多态。泛型主要目的是为程序员提供编程的便利，减少代码的臃肿，同时可以极大地丰富语言本身的表达能力，为程序员提供了一个合适的炮管。想想，一个函数，可以代替几十个，甚至数百个函数，是一件多么让人兴奋的事情：
// *
// */
// fn add<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
//     a * b
// }

// // fn largest<T>(list: &[T]) -> T {
// //     let mut largest = list[0];
// //     for &item in list.iter() {
// //         if item > largest {
// //             largest = item;
// //         }
// //     }
// //     largest
// // }

// // 结构体中使用泛型定义
// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// // 如果想让 x 和 y 既能类型相同，又能类型不同，就需要使用不同的泛型参数：
// struct Point2<T, U> {
//     x: T,
//     y: U,
// }

// // 在枚举中使用泛型
// enum Option<T> {
//     Some(T),
//     None,
// }

// // 在方法中使用泛型
// struct Point6<T> {
//     x: T,
//     y: T,
// }

// struct Point99<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point6<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// // 为具体的泛型类型提供方法
// impl Point99<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// // const 泛型
// // [i21; 2] 和 [i21; 3] 是不同的数组类型
// fn display_array(arr: &[i32]) {
//     println!("{:?}", arr);
// }

// fn display_array2<T: std::fmt::Debug>(arr: &[T]) {
//     println!("{:?}", arr);
// }

// // usize 什么意思

// fn const_arr<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let integer = Some(5);
//     let float = Some(5.0);
//     println!("{:?}", integer);
//     println!("{:?}", float);
//     let arr: [i32; 3] = [1, 2, 3];
//     const_arr(arr);

//     let arr: [i32; 2] = [1, 2];
//     const_arr(arr);
//     display_array(&[1, 2, 3]);
//     display_array(&[1, 2]);
//     // 只要使用切片数组 传入arr 的不可变引用就行
//     // 使用泛型
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     display_array2(&arr);

//     let arr: [i32; 2] = [1, 2];
//     display_array2(&arr);

//     // 实现 const 泛型

//     let num: u64 = 55;
//     println!("{}", num.pow(2));
//     let p1 = Point99 { x: 5.0, y: 10.0 };
//     println!("p1.x = {}", p1.distance_from_origin());
//     let p = Point6 { x: 10, y: 600 };
//     println!("p.x = {}", p.x());
//     let number_list = vec![34, 50, 25, 100, 65];
//     // println!("The largest number is {}", result);
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     println!("{:?}", float);
//     // 必须要使用相同的类型 不然会报错
//     // let error = Point { x: 1.0, y: 4 };

//     println!("add generics: {}", add(6, 200));
//     println!("add generics: {}", add(1.5, 5.1));
//     println!("add generics: {}", add(800, 200));
//     println!("add i8: {}", add_i8(1, 2));
//     println!("add i8: {}", add_i8(20i8, 3i8));
//     println!("add i32: {}", add_i32(800, 600));
//     println!("add i32: {}", add_i32(800i32, 600i32));
//     println!("add f64: {}", add_f64(1.2, 3.4));
// }

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let res = get_largest(number_list);
    println!("The largest number is {}", res);

    enum Options<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
