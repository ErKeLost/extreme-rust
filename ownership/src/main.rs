// fn main() {
//     println!("Hello, world!");
// }
// 栈和堆都是代码在运行时可供使用的内存，但是它们的结构不同。
// 栈以放入值的顺序存储值并以相反顺序取出值。这也被称作 后进先出
// （last in, first out）。想象一下一叠盘子：当增加更多盘子时
// ，把它们放在盘子堆的顶部，当需要盘子时，也从顶部拿走。
// 不能从中间也不能从底部增加或拿走盘子！增加数据叫做 进栈
// （pushing onto the stack），而移出数据叫做 出栈
// （popping off the stack）。
// fn main() {
//     let mut ui = "12312121223"
//     let mut s = String::from("hello");
//     let mut stack = String::from("666");
//     stack.push_str(", world!"); // push_str() 在字符串后追加字面值

//     println!("{}", s); // 将打印 `hello, world!`
// }

// fn main() {
//     let x = 5;
//     let y = x;
//     println!("{}", y)
// }

// 我们大致可以猜到这在干什么：“将 5 绑定到 x；接着生成一个值 x 的拷贝并绑定到 y”。现在有了两个变量，x 和 y，都等于 5。这也正是事实上发生了的，因为整数是有已知固定大小的简单值，所以这两个 5 被放入了栈中。
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{}, world!", s1);
// }
// rust 禁止使用无效引用无效

// 复制
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);
// }
// fn main() {
//     let s = String::from("hello"); // s 进入作用域

//     takes_ownership(s); // s 的值移动到函数里 ...
//                         // ... 所以到这里不再有效
//     // println!("{}", s);
//     let x = "asdasd"; // x 进入作用域

//     makes_copy(x); // x 应该移动函数里，
//     println!("{}", x);
//     // 但 i32 是 Copy 的，所以在后面可继续使用 x
// } // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
//   // 所以不会有特殊操作

// fn takes_ownership(some_string: String) {
//     // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// fn makes_copy(some_integer: &str) {
//     // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。不会有特殊操作

// 返回值 与 作用域

// fn main() {
//     let s1 = String::from("hello world");
//     let s2 = gives();
//     println!("{}", s2);
//     let s3 = takes_and_gives_back(s1);
//     println!("{}", s3);
//     println!("{}", s1);
// }

// fn gives() -> String {
//     let str = String::from("owner");
//     str
// }
// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string 进入作用域

//     a_string // 返回 a_string 并移出给调用的函数
// }

// // 解析
// fn main() {
//     let s1 = gives_ownership(); // gives_ownership 将返回值
//                                 // 移给 s1

//     let s2 = String::from("hello"); // s2 进入作用域

//     let s3 = takes_and_gives_back(s2); // s2 被移动到
//                                        // takes_and_gives_back 中,
//                                        // 它也将返回值移给 s3
// } // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
//   // 所以什么也不会发生。s1 移出作用域并被丢弃

// fn gives_ownership() -> String {
//     // gives_ownership 将返回值移动给
//     // 调用它的函数

//     let some_string = String::from("yours"); // some_string 进入作用域

//     some_string // 返回 some_string 并移出给调用的函数
// }

// // takes_and_gives_back 将传入字符串并返回该值
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string 进入作用域

//     a_string // 返回 a_string 并移出给调用的函数
// }

// 可以使用元组
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
