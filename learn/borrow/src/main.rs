// fn main() {
//     println!("Hello, world!");
// }

// 借用与引用
// 示例 4-5 中的元组代码有这样一个问题：我们必须将 String 返回给调用函数，以便在调用 calculate_length 后仍能使用 String，因为 String 被移动到了 calculate_length 内。

// 下面是如何定义并使用一个（新的）calculate_length 函数，它以一个对象的引用作为参数而不是获取值的所有权：
// 这些 & 符号就是 引用，它们允许你使用值但不获取其所有权。
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
// fn calculate_length(s: &String) -> usize {
//     // s 是对 String 的引用
//     s.len()
// } // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
//   // 所以什么也不会发生

// fn main() {
//     let s1 = String::from("hello");
//     let res: usize = test(&s1);
//     println!("{} {}", s1, res);
// }

// fn test(s: &String) -> usize {
//     s.len()
// }

//   不能尝试修改 借用的变量 引用就是借用

// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// 变量 跟 借用一样 都是不可变的

// 可变引用

// fn main() {
//     let mut s = String::from("ooo");
//     set_borrow(&mut s);
//     println!("{}", s)
// }

// fn set_borrow(self_a: &mut String) {
//     self_a.push_str("world");
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
//     println!("{}", s)
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// 重复学习可变引用

// fn main() {
//     let mut s = String::from("我是可变的引用");
//     change_mut(&mut s);
//     println!("{}", s);
// }
// fn change_mut(copy_string: &mut String) {
//     copy_string.push_str("我发生改变了啊")
// }

// 在同一时间，只能有一个对某一特定数据的可变引用

// fn main() {
//     let mut s = String::from("current");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{}, {}", r1, r2);
// }

// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

//     let r2 = &mut s;
// }

// 可变 与 不可变
// fn main() {
//     let mut s = String::from("6666");
//     let a = &mut s;
//     a.push_str("5476887986");
//     println!("{}", s);
// }

// fn main() {
//     let mut s = String::from("123");
//     let a = s;
//     println!("{}", s);
// }

// fn main() {
//     let mut s = String::from("hello");

//     let mut r1 = &s; // 没问题
//     let r2 = &s; // 没问题
//     println!("{} and {}", r1, r2);
//     // 此位置之后 r1 和 r2 不再使用

//     let r3 = &mut s; // 没问题
//     println!("{}", r3);
// }

// 悬垂引用

fn main() {
    let danger_string = get_danger_string();
    println!("{}", danger_string);
}
// TODO 这里 s 离开作用域并被丢弃。其内存被释放
// fn get_danger_string() -> &String {
//     let s = String::from("hello");
//     &s
// }
fn get_danger_string() -> String {
    let s = String::from("hello erkelost");
    s
}
