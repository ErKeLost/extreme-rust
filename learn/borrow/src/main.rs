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

// fn main() {
//     let danger_string = get_danger_string();
//     println!("{}", danger_string);
// }
// // TODO 这里 s 离开作用域并被丢弃。其内存被释放
// // fn get_danger_string() -> &String {
// //     let s = String::from("hello");
// //     &s
// // }
// fn get_danger_string() -> String {
//     let s = String::from("hello erkelost");
//     s
// }

// #[derive(Debug)]
fn main() {
    println!("Hello, world!");

    // 所有权
    // 所有程序都需要和 计算机内存 打交道 如何申请内存来存放程序的运行内容 如何在不需要的时候释放空间
    // 所有编程语言设计的难点之一 就是如何管理内存
    // - 垃圾回收机制 在程序的运行中不断寻找不再使用的内存空间 并释放
    // - 手动管理内存的分配和释放 在程序中 通过函数调用的方式来申请和释放内存
    // - 通过所有权来管理内存 编译器在编译时会根据一系列的规则进行检查

    // rust 选择第三种 而且这种检查只发生在编译期 在程序运行的时候 不会有任何性能问题

    // 对于在堆上存放的数据 是未知大小或者可能变化的数据
    // 当在堆上存放数据是 需要请求一定大小的内存地址 操作系统在堆上找到一块足够大的空间  把他标记为已经使用
    // 并且返回一个表示该位置地址的指针  该过程被称为在堆上分配内存 简称分配

    // 接着 返回的指针推入栈中 因为指针的大小是已知并且固定的 在后续使用中 你将通过在栈中的指针来获取在堆上的实际内存

    // 然后才能访问数据 由此可知 堆是一种缺乏组织的数据结构 想象一下去餐馆吃饭 你告诉服务员有几个人 服务员找一个足够大的

    // 桌子 堆上分配的内存空间

    // 性能上 cpu 高速缓存 栈数据可以直接存储在 cpu 的高速缓存上 堆数据只能存在内存中 访问堆上的数据比访问栈上的数据慢

    // 处理器处理栈上的数据会比处理堆上的数据更加高效

    // string 是一个复杂类型 由存储在栈中的堆指针 字符串长度 和 字符串容量 共同组成 堆指针指向真实存储的字符串内容的堆内存

    // 容量是堆内存分配空间的大小，长度是目前已经使用的大小。

    // 假定一个值可以有两个所有者 当变量离开作用域时 rust 自动调用 drop 函数 并且清理变量的内存 
}
