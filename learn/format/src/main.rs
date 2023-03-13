fn main() {
    println!("Hello"); // => "Hello"
    println!("Hello, {}!", "world"); // => "Hello, world!"
    println!("The number is {}", 1); // => "The number is 1"
    println!("{:?}", (3, 4)); // => "(3, 4)"
    println!("{value}", value = 4); // => "4"
    println!("{} {}", 1, 2); // => "1 2"
    println!("{:04}", 42); // => "0042" with leading zeros
}

// println 宏接收的是一个可变参数，
//  第一个参数是一个字符串常量,
// 他表示最终输出到终端的字符串格式 包含 {} 符号是占位符
// ，用来表示后面的参数的值
// 会被 println后面的参数移除替换
