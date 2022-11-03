// fn main() {
//     println!("Hello, world!");
// }

// 借用与引用
// 示例 4-5 中的元组代码有这样一个问题：我们必须将 String 返回给调用函数，以便在调用 calculate_length 后仍能使用 String，因为 String 被移动到了 calculate_length 内。

// 下面是如何定义并使用一个（新的）calculate_length 函数，它以一个对象的引用作为参数而不是获取值的所有权：
// 这些 & 符号就是 引用，它们允许你使用值但不获取其所有权。
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
