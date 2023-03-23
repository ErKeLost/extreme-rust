fn main() {
    let a: i32 = 10;
    let b: u16 = 100;
    // if a < b {
    //     println!("a is smaller than b");
    // }
    // 这段代码会报错 因为 a 和 b 的类型不同
    // 可以通过类型转换 使用 as 操作符来解决
    // 一般都是把比较小的范围类型转换成比较大的范围类型 范围较大 不是 数值较大

    if a < b as i32 {
        println!("a is smaller than b");
    }
}
