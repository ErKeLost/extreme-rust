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

// #[derive(Debug)]
// enum PokerSuit {
//     Clubs(u8),
//     Spades(u8),
//     Diamonds(u8),
//     Hearts(u8),
// }
// fn main() {
//     let heart = PokerSuit::Hearts(88);
//     let diamond = PokerSuit::Diamonds(99);

//     // print_suit(heart);
//     // print_suit(diamond);

//     match heart {
//         PokerSuit::Clubs(value) => println!("Clubs {}", value),
//         PokerSuit::Spades(value) => println!("Spades {}", value),
//         PokerSuit::Diamonds(value) => println!("Diamonds {}", value),
//         PokerSuit::Hearts(value) => println!("Hearts {}", value),
//     }

//     match diamond {
//         PokerSuit::Clubs(value) => println!("Clubs {}", value),
//         PokerSuit::Spades(value) => println!("Spades {}", value),
//         PokerSuit::Diamonds(value) => println!("Diamonds {}", value),
//         PokerSuit::Hearts(value) => println!("Hearts {}", value),
//     }
// }

// fn print_suit(card: PokerSuit) {}

enum IpAddrKind {
    V4(String),
    V6(String),
    V8(u8, u8, u8, u8),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let localhost = IpAddrKind::V6(String::from("::1"));

    let x: i8 = 32;

    let y = Some(66);

    let sum = x + y.unwrap();

    println!("{}", sum);

    let some = Some(5);

    assert_eq!(some.is_some(), true);

    // unwrap 方法 返回 option 包含的值 如果变量为空 则会导致 panic

    let some_number = Some(5);

    // 下面的代码将会 panic
    let none_number: Option<i32> = None;
    // let result1 = none_number.unwrap();

    let result2 = some_number.unwrap_or(0);
    let result3 = none_number.unwrap_or(0);

    assert_eq!(result2, 5);
    assert_eq!(result3, 0);
    //  or else 和 unwrap_or_else 的区别在于 unwrap_or_else 接受一个闭包

    // map() 方法接受一个函数作为参数，用于将 Option<T> 中的值转换为另一种类型。如果 Option<T> 变量为空，则调用函数不会发生任何操作。

    let some_number = Some(5);
    let none_number: Option<i32> = None;

    let result1 = some_number.map(|x| x * 2);
    let result2 = none_number.map(|x| x * 2);

    assert_eq!(result1, Some(10));
    assert_eq!(result2, None);

    let some_number = Some(Some(5));
    let none_number: Option<Option<i32>> = None;

    let result1 = some_number.flatten();
    let result2 = none_number.flatten();
    println!("{:?}", result1);
    println!("{:?}", some_number);
    assert_eq!(result1, Some(5));
    assert_eq!(result2, None);

    let some_number = Some(5);
    let none_number: Option<i32> = None;

    let result1 = some_number.filter(|x| x > &2);
    let result2 = some_number.filter(|x| x > &10);
    let result3 = none_number.filter(|x| x > &2);

    assert_eq!(result1, Some(5));
    assert_eq!(result2, None);
    assert_eq!(result3, None);
}
