use std::io; // 使用标准库中的 io 这个模块

// fn main() {
//     println!("Hello, world!");
//     println!("Please input a number: ");
//     let mut input = String::new(); // 在这里我们创建了一个新的 String，用来接收下面的输入
//     io::stdin()
//         .read_line(&mut input) // 读取一行
//         .expect("Failed to read input!"); // 比较粗暴的错误处理
//     println!("Your raw input is: {:?}.", input); // 打印输入的原始内容
//     let number: i64 = input.trim().parse().expect("Input is not a number!"); // trim 把前后的空格、换行符这些空白字符都去掉，parse 将输入的字符串解析为 i64 类型，如果解析失败就报错
//     println!("Your input is: {}.", number); // 打印 parse 之后的 i64 数字
// }

fn main() {
    println!("请输入要计算 π 的小数位数：");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    let n: usize = input.trim().parse().expect("解析数字失败");
    let pi_approx = format!("{:.width$}", std::f64::consts::PI, width = n);
    println!("π 的前 {} 位小数：{}", n, pi_approx);
}
