#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    // 创建动态数组
    // 使用 Vec::new 创建动态数组是创建rusty的方式 调用了 Vec 的new 关联函数

    // let v: Vec<i32> = Vec::new();
    // let mut v = Vec::new();
    // v.push(1);
    // println!("{:?}", v)

    // // 还可以使用宏来创建 vec 数组 与 vec::new 不同 这个可以创建的时候同时初始化
    // let r = vec![1, 2, 3];
    // println!("{:?}", r);

    // // 尾部添加元素
    // let mut w = vec![1, 2, 3];
    // w.push(555);

    // 跟其他结构体一样
    // vector 超出作用域范围之后会被自动删除

    // 从 vector 中读取元素
    let e = vec![1, 2, 3, 4, 5];
    let third: i32 = e[2];
    println!("The third element is {}", third);

    match e.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }

    // 同时借用多个数组元素

    let mut res = vec![1, 2, 3, 4, 5];

    let first = res[0];

    res.push(6);
    println!("{:?}", res);
    println!("The first element is: {}", first);

    // 遍历 vector中的元素

    let v = vec![1, 2, 3];
    for i in v {
        println!("{i}");
    }

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 10
    }

    println!("{:?}", v);

    // 存储不同类型的元素

    let v = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];

    for ip in v {
        show_addr(ip)
    }

    fn show_addr(ip: IpAddr) {
        println!("{:?}", ip)
    }
}
