// fn main() {
//     println!("Hello, world!");
// }

// match 控制流运算符
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main() {
//     let w = Coin::Penny;
//     let mut s = value_in_cents(w);
//     println!("{}", s);
// }

// match 可以返回任意类型的值做判断
// 走不同分支 但是 if 只能返回 bool

// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> usize {
//     match coin {
//         Coin::Penny => {
//             println!("真穷啊");
//             111
//         }
//         Coin::Nickel => {
//             println!("你也挺穷啊");
//             222
//         }
//         Coin::Dime => {
//             println!("你以为你是啥啊");
//             666
//         }
//         Coin::Quarter(state) => {
//             println!("就你跟别人不一样啊 ？{:?}", state);
//             999
//         }
//     }
// }

// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
// }
// fn main() {
//     fn plus_one(x: Option<i32>) -> Option<i32> {
//         match x {
//             None => None,
//             Some(i) => Some(i + 1),
//         }
//     }

//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("{:?}", none);
// }
// Some(i) => Some(i + 1),
// Some(i) === Some(5) 他们表示同一个成员

// 匹配是穷尽的 match 匹配是穷尽的

// 让我们改变游戏规则，当你掷出的值不是 3 或 7 的时候，
// 你必须再次掷出。这种情况下我们不需要使用这个值，
// 所以我们改动代码使用 _ 来替代变量 other ：
// 通配符 _ 会报一个不存在的变量
// fn main() {
//     let dice_roll = 9999;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),
//         _ => reroll(),
//         // 无事发生
//         _ => (),
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn move_player(num_spaces: u128) {
//         println!("{}", num_spaces)
//     }
//     fn reroll() {}
// }

// if let 简单控制流

// fn main() {
//     let some = Some(3);
//     match some {
//         Some(3) => println!("three"),
//         _ => (),
//     }
//     if let Some(3) = some {
//         println!("我是some3")
//     }
// }

// if let 可以包含在match 表达式中
// 可以加在 else  if let 就是 match 的语法糖

// #![allow(unused)]
// fn main() {
//     #[derive(Debug)]
//     enum UsState {
//         Alabama,
//         Alaska,
//     }

//     enum Coin {
//         Penny,
//         Nickel,
//         Dime,
//         Quarter(UsState),
//     }
//     let coin = Coin::Penny;
//     let mut count = 0;
//     match coin {
//         Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//         _ => count += 1,
//     }
//     if let Coin::Quarter(state) = coin {
//         println!("State quarter from {:?}!", state);
//     } else {
//         count += 1;
//     }
// }
