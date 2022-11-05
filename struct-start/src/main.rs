// struct User {
//     name: String,
//     sex: bool,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let user1 = User {
//         email: String::from("woshinidie"),
//         sex: false,
//         name: String::from("nizhenniubi"),
//         sign_in_count: 99,
//     };
//     println!("{}", user1.sex);
// }

// 设置一个可变的结构体实例

// fn main() {
//     // let mut user1 = User {
//     //     email: String::from("someone@example.com"),
//     //     name: String::from("someusername123"),
//     //     sex: false,
//     //     sign_in_count: 1,
//     // };

//     // user1.email = String::from("anotheremail@example.com");
//     let s = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );
//     println!("{}", s.email);
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         name: username,
//         sex: true,
//         sign_in_count: 1,
//     }
// }
// // 字面量语法简写
// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// 使用结构体更新语法 从其他实例中创建新的实例
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     // --snip--

//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     // let user2 = User {
//     //     active: user1.active,
//     //     username: user1.username,
//     //     email: String::from("another@example.com"),
//     //     sign_in_count: user1.sign_in_count,
//     // };
//     // 这种情况我们可以使用user1
//     let user2 = User {
//         email: String::from("anoadasdsadadther@example.com"),
//         ..user1
//     };
//     // 这种情况 我们就相当于 把user1 交给了 user2 我们就获取不到user1了
//     let user2 = User {
//         // email: String::from("anoadasdsadadther@example.com"),
//         ..user1
//     };
//     // 请注意，结构更新语法就像带有 = 的赋值，因为它移动了数据
//     println!("{}", user2.username);
//     println!("{}", user2.email);
//     println!("{}", user1.email);
// }

// 元祖结构体
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     println!("{} {} {}", origin.0, origin.1, origin.2);
// }

// 没有任何字段的类单元结构体
// struct AlwaysEqual;

// fn main() {
//     let subject = AlwaysEqual;
//     println!("{} ", subject);
// }

// 结构体程序实例

// fn main() {
//     let width = 30;
//     let height = 50;
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width, height)
//     );
// }
// fn area(width: usize, height: usize) -> usize {
//     width * height
// }

// 程序没有把这两个参数之间的关联体现出来
// 使用元组重构
// fn main() {
//     let rect = (30, 50);
//     println!("The area of the rectangle is {} square pixels.", area(rect))
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// 元组帮助我们增加了一些结构性 这样我们只需要传递一个参数
// 但是函数中的结构 却让人费解了 你不知道 这个 0 1 到底指的是什么

// 使用结构体重构 赋予更多的意义

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let react = Rectangle {
//         width: 30,
//         height: 600,
//     };
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&react)
//     );
//     println!("{}", react.height);
// }
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// 调试结构体派生 trait
// 示例 5-12：增加属性来派生 Debug trait，并使用调试格式打印 Rectangle 实例s
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     // println!("rect1 is {:?}", rect1);
//     println!("rect1 is {:#?}", rect1);
//     dbg!(&rect1);
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

// 方法语法 方法语法与函数类似 他们使用fn 声明 可以拥有参数和返回值

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

// 方法都要到 impl中
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn width(&self) -> bool {
//         self.width > 0
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }
// 我们可以在同名的方法中使用一个字段。
// 我们可以在同名的方法中使用一个字段来达到任何目的
// 。在 main 中，当我们在 rect1.width 后面加上括号时。
// Rust 知道我们指的是方法 width。当我们不使用圆括号时，
// Rust 知道我们指的是字段 width。

// 多个参数
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// 所有在impl中定义的函数被称为关联函数
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     // fn square(size: u32) -> Rectangle {
//     //     Rectangle {
//     //         width: size,
//     //         height: size,
//     //     }
//     // }
//     fn square(size: u32) -> u16 {
//         123
//     }
// }

// fn main() {
//     let sq = Rectangle::square(3);
//     println!("square {:#?}", sq);
// }

// 多个 impl 块

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
