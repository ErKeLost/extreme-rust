fn main() {
    // 字节串的类型是字节的数组，而不是字符串了
    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);

    // 可以看看下面这串打印出什么
    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    // 字节串与原始字面量结合使用
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    let arr = [0x52, 0x75, 0x73, 0x74];
    println!("{:?}", arr[0]);
    // println!("{:?}", arr[6]);

    // let v: Vec = Vec::new();
    // let v = vec![1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v[1]);

    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // 标准的函数定义
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    // 闭包的定义，请注意形式对比
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // 闭包的定义2，省略了类型标注
    let add_one_v3 = |x| {x + 1};
    // 闭包的定义3，花括号也省略了
    let add_one_v4 = |x| x + 1;
}
