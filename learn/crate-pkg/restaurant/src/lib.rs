// // #![allow(unused)]
// // fn main() {
// //     mod front_of_house {
// //         mod hosting {
// //             fn add_to_waitlist() {}

// //             fn seat_at_table() {}
// //         }

// //         mod serving {
// //             fn take_order() {}

// //             fn server_order() {}

// //             fn take_payment() {}
// //         }
// //     }
// // }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("我跑起来了啊")
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }
// // 在绝对路径，我们从 crate，也就是 crate 根部开始。
// // 然后 crate 根部中定义了 front_of_house 模块。
// // front_of_house 模块不是公有的，
// // 不过因为 eat_at_restaurant 函数与 front_of_house
// // 定义于同一模块中（即，eat_at_restaurant 和 front_of_house
// //  是兄弟），我们可以从 eat_at_restaurant 中引用
// //   front_of_house。接下来是使用 pub 标记的 hosting 模块。
// //   我们可以访问 hosting 的父模块，所以可以访问 hosting。最后，
// //   add_to_waitlist 函数被标记为 pub ，我们可以访问其父模块
// //   ，所以这个函数调用是有效的！

// // 使用super构建从父模块开始的相对路径
// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// // 创建公有的结构体和枚举

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal
//     // meal.seasonal_fruit = String::from("blueberries");
// }

// // 结构体 toast 字段是公有的 所以可以在 其他 函数中随意使用 但是不能使用别的字段

// // 枚举设置为共有的 那么所有属性都是共有的

// // 使用 use 创建软连接

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }
// fn main() {}

// // 使用 as 别名

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }
mod front;

pub use crate::front::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
