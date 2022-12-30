/** vector 用来存储一系列的值 类型 vec<T>
 vector 只能存放相同类型的值 在一些拥有一系列项
的场景中非常实用*/

// 创建一个 空的 vector

#[allow(unused)]
fn main() {
    let v: Vec<i32> = Vec::new();
    // 宏 一个包含初始值的vector
    let b = vec![1, 2, 3];
    // 修改 vector
    // let mut c: Vec<i8> = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    // 丢弃 vector 时也会丢弃其所有元素
    // 离开作用域 元素就会被释放
    {
        let v = vec![1, 2, 3, 4];

        // 处理变量 v
    } // <- 这里 v 离开作用域并被丢弃
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
