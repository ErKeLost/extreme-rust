fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s); // 打印 `hello, world!`

    let s1: String = String::from("hello");
    println!("{}", s1);

    let sr: &str = "hello";
    println!("{}", sr);
    {
        // 规则 2：一个值同时只能有一个所有者
        let mut s2 = s1; // 所有权从 s1 转移到 s2（s1 不再有效）
                         // println!("{}", s1); // 这会导致编译时错误，因为 s1 不再有效
        println!("{}", s2); // 这是允许的，因为 s2 现在拥有该值
    }

    let mut s1 = String::from("hello");
    let s2 = s1.push_str("223132").clone();

    println!("s1 = {}, s2 = {:?}", s1, s2);
    let s = String::from("hello");
    takes_ownership(s); // s转移到了函数内，不再可用

    // s 不再可用

    let s = gives_ownership(); // s 获得了返回值的所有权
    let mut s = String::from("hello");
    change(&mut s);
    println!("The updated string is: {}", s);

    // 对于一个变量来说 同时只能存在一个可变引用或者多个不可变引用

    // 特殊的引用 切片
    // 切片是一种特殊的引用 所以没有所有权
}

fn takes_ownership(s: String) {
    println!("Received string: {}", s);
} // s 离开作用域，被丢弃

fn gives_ownership() -> String {
    String::from("hello")
} // 返回了String的所有权

fn change(s: &mut String) {
    s.push_str(", world!");
}
