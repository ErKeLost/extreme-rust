fn main() {
    println!("Hello, world!");
    let a = 123;
    println!("a = {}", a);
    // 将""号进行转义
    let byte_escape = "I'm saying \"Hello\"";
    println!("{}", byte_escape);

    // 分两行打印
    let byte_escape = "I'm saying \n 你好";
    println!("{}", byte_escape);

    // Windows下的换行符
    let byte_escape = "I'm saying \r\n 你好";
    println!("{}", byte_escape);

    // 打印出 \ 本身
    let byte_escape = "I'm saying \\\\ // Ok";
    println!("{}", byte_escape);

    // 强行在字符串后面加个0，与C语言的字符串一致。
    let byte_escape = "I'm saying hello.\0";
    println!("{}", byte_escape);

    let byte_escape = "I'm saying hello \x7f";
    println!("{}", byte_escape);

    let byte_escape = "I'm saying hello \u{0065}";
    println!("{}", byte_escape);

    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
    // 这个字面量必须使用r##这种形式，因为我们希望在字符串字面量里面保留""
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    let bytestring: &[u8; 21] = b"this is a byte string";
    println!("A byte string: {:?}", bytestring);
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let x = (500, 600, 700);
    println!("x.0 = {}", x.0);

    let x = 1;
    // 在这里，if else 返回了值
    let y = if x == 0 {
        // 代码块结尾最后一句不加分号，表示把值返回回去
        100
    } else {
        // 代码块结尾最后一句不加分号，表示把值返回回去
        101
    };
    println!("y is {}", y);

    let mut counter = 0;

    // 这里，接收从循环体中返回的值，对result进行初始化
    let result = loop {
        counter += 1;
        println!("counter = {}", counter);
        if counter == 10 {
            // 使用break跳出循环，同时带一个返回值回去
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    for number in 1..4 {
        println!("{number}");
    }
    println!("--");
    // 左闭右闭区间
    for number in 1..=4 {
        println!("{number}");
    }
    println!("--");
    // 反向
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("------");

    for item in 'a'..'z' {
        println!("{item}");
    }

    fn user_name(name: &str) {
        println!("name is {name}");
    }
    user_name("zhangsan");
    let user_name = |name: &str| {
        println!("name is {name}");
    };

    user_name("lisi");
}
