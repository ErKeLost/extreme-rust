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
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
