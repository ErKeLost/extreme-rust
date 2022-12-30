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

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 30,
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


结构体 struct 复合数据结构 它由其他数组类型组合而来 跟其他语言的 object record 差不多 就是 interface

但是他是数据结构 不是 ts 中的类型

定义结构体

一个结构体由几部分组成

通过关键字 struct 定义

一个清晰明确的结构体

几个有名字的结构体

结构体定义某网站的用户

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

该结构体名称是 User 拥有四个字段 并且每个字段都有对应的字段名称和类型声明 

为了使用上述结构体 我们就需要创建对应结构体的实例 

let use1 = User  {
    email: String::from("erkelost@example.com"),
    avtive: false,
    username: String::from("erkelost"),
    sign_in_count: 1
}

创建一个结构体意思就是创建一个对象

初始化实例的时候 每一个字段都需要进行初始化

初始化定义字段的书讯不需要和定义结构体顺序一致

当然也可以修改结构体

但是你修改的时候 必须要把结构体变成可变的

let mut user2 = User {
    email: String::from("erkelost@example.com"),
    avtive: false,
    username: String::from("erkelost"),
    sign_in_count: 1
}

user2.email = String::from("localhost:8080")

但是你不可以单独修改其中的一个字段 只能对整个结构体标记为可变


简化结构体的创建 类似 js 中的构造函数

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        // 可以简化 跟js一样 对象的扩展写法
        active: true,
        sign_in_count: 10
    }
}

结构体更新语法 也可以像js一样写...
但是rust 是 ..
let user2 = User {
    email: String::from("213123"),
    ..user1
} 
这种情况下 user1 将会在无法使用 但是只会有 username字段发生了 所有权转移

因为所有权的 copy 特征 bool 和 u64 都实现了 copy 所以仅仅是发生了拷贝， 而不是所有权转移

虽然 user1 不能被使用 但是 不代表他里面的其他字段不可以被使用 copy 字段可以继续使用


元组结构体 tuple struct

结构体必须要有名称， 但是结构体的字段可以没有名称 这样就长得像元组 因此被叫做元组结构体


struct Color(i32, i32, i32);

struct Color(i32, i32, i32);

let black = Color(0,0,0);

let origin = Point(0,0,0)

单元结构体 不关系字段数据 只关心他的行为 他爱咋写咋写

struct AlwaysEqual;

let subject = AlwaysEqual;

// 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
impl SomeTrait for AlwaysEqual {

}

结构体是拥有它所有的数据 而不是从其他地方借用数据
用 String::from 不是 &str



使用 #[derive(Debug)] 来打印结构体的信息

这样才可以使用 println!("{:?}", s)

结构体没有实现display特征 所以会有这种情况

结构体较大时，希望有更好的输出表现 可以使用 {:#?} 来代替 {:?}

还有一个简单的输出 debug 信息的方法，那就是使用 dbg! 宏，
它会拿走表达式的所有权，然后打印出相应的文件名、行号等 debug 信息，
当然还有我们需要的表达式的求值结果。
除此之外，它最终还会把表达式值的所有权返回！
