// 使用字符串存储 UTF-8 编码的文本

#[warn(unused_attributes)]
fn main() {
    let mut s = String::new();
    let data = "hello world";
    let s = data.to_string();
    let s = "erkelost".to_string();
    let b = "adny";
    println!("{} {}", s, b);
    test();
    aa();
}
// String::new() 创建一个字符串
// String::from() 使用字符串字面量创建string
// String::from("erkelost") = "erkelost".to_string();

// 字符串是UTF-8编码的 所有可以包含任何正确的编码

// let hello = String::from("السلام عليكم");
// let hello = String::from("Dobrý den");
// let hello = String::from("Hello");
// let hello = String::from("שָׁלוֹם");
// let hello = String::from("नमस्ते");
// let hello = String::from("こんにちは");
// let hello = String::from("안녕하세요");
// let hello = String::from("你好");
// let hello = String::from("Olá");
// let hello = String::from("Здравствуйте");
// let hello = String::from("Hola");

// 可以使用 push_str 和 push 来增加字符串 是 String变长

fn test() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    let a = String::from("666");
    let b = String::from("666");
    let c = a + &b;
    println!("{}", c);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}
// add 函数 后面的参数 必须是 引用字符串
// push_str 是 push 字符串 push 是传递一个字符

// 遍历字符串的方法
// 幸运的是，这里还有其他获取字符串元素的方式。

// 如果你需要操作单独的 Unicode 标量值，
// 最好的选择是使用 chars 方法。对 “नमस्ते” 调用
//  chars 方法会将其分开并返回六个 char 类型的值，
//  接着就可以遍历其结果来访问每一个元素了：

fn aa() {
    for c in "erkelost".chars() {
        println!("{}", c);
    }
    for c in "erkelost".bytes() {
        println!("{}", c);
    }
}
