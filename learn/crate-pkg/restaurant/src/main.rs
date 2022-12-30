use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io;
use std::io::Write;
use std::{cmp::Ordering, io};
// 语法 在嵌套路径中使用 self
use std::io::{self, Write};
// 所有公有 引入作用域 可以用 * glob运算符
use std::collections::*;
// 优化 代码 引用 写成大括号
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("secret_number: {}", secret_number);
}


// 将模块分割进不同文件



