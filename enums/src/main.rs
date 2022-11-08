// fn main() {
//     println!("Hello, world!");
// }

// 枚举和模式匹配
// 本章介绍 枚举（enumerations），也被称作 enums。
// 枚举允许你通过列举可能的 成员（variants） 来定义一个类型。
// 首先，我们会定义并使用一个枚举来展示它是如何连同数据一起编码信息的。
// 接下来，我们会探索一个特别有用的枚举，叫做 Option，
// 它代表一个值要么是某个值要么什么都不是。
// 然后会讲到在 match 表达式中用模式匹配，针对不同的枚举值编写相应要执行的代码。
// 最后会介绍 if let，另一个简洁方便处理代码中枚举的结构。

// 枚举是一个很多语言都有的功能，不过不同语言中其功能各不相同。
// Rust 的枚举与 F#、OCaml 和 Haskell 这样的函数式编程语言中的
//  代数数据类型（algebraic data types）最为相似。

// enum IpAddrKind {
//     V4,
//     V6,
// }
// fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     println!("{:?}", four);
//     println!("{:?}", six);
//     route(six)
// }

// // 定义一个函数来获取任何 ipaddrkind

// fn route(id_type: IpAddrKind) {
//     println!("{:?}", id_type);
// }

// 把 枚举 存在结构体里面
// #[derive(Debug)]
// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }

//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
//     println!("{:?#}", loopback);
//     println!("{:?#}", home);
// }

// 我们可以用 枚举将数据直接放进每一个枚举成员而不是当作结构体的一部分
// #![allow(unused)]
// fn main() {
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }

//     let home = IpAddr::V4(String::from("127.0.0.1"));

//     let loopback = IpAddr::V6(String::from("::1"));
// }

// 枚举类型可以存放任意类型的数据 字符串 数字类型 结构体
// 甚至可以包含哪一个枚举

// #![allow(unused)]
// #[derive(Debug)]
// fn main() {
//     enum Message {
//         Quit,
//         Move { x: i32, y: i32 },
//         Write(String),
//         ChangeColor(i32, i32, i32),
//     }
//     println!("{}", Message)
// }

// 枚举 option<T> 定义于标准库 不需要直接显示的引用他们

// #![allow(unused)]
// #[derive(Debug)]
// fn main() {
//     let some_number = Some(5);
//     let some_string = Some("a string");

//     let absent_number: Option<i32> = None;
//     println!("{:?#}", some_number)
// }
