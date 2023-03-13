// println 宏接收的是一个可变参数，
//  第一个参数是一个字符串常量,
// 他表示最终输出到终端的字符串格式 包含 {} 符号是占位符
// ，用来表示后面的参数的值
// 会被 println后面的参数移除替换

// print! 将格式化文本输出到标准输出，不带换行符
// println! 同上，但是在行的末尾添加换行符
// format! 将格式化文本输出到 String 字符串

use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    println!("Hello"); // => "Hello"
    println!("Hello, {}!", "world"); // => "Hello, world!"
    println!("The number is {}", 1); // => "The number is 1"
    println!("{:?}", (3, 4)); // => "(3, 4)"
    println!("{value}", value = 4); // => "4"
    println!("{} {}", 1, 2); // => "1 2"
    println!("{:04}", 42); // => "0042" with leading zeros

    // 最常用的 是 println! format!
    let s = "hello";
    println!("S is {}", s);

    // format! 返回一个字符串

    let s1 = format!("{} world", s);
    print!("{}\r\n", s1);
    print!("{}\n", "!");

    // 它们仅应该被用于输出错误信息和进度信息，其它场景都应该使用 print! 系列。
    eprintln!("Error: Could not complete task");

    // 与 其他语言 go 不同 %d %s rust 特立独行选择了 {} 作为格式化占位符

    // 与 {} 类似 {:?} 也是占位符

    // {} 适用于实现了 std::fmt::Display 特征的类型 用来更优雅 友好的格式化文本

    // {:?} 适用于 std::fmt::Debug 用于调试的场景

    // 两者的选择很简单 需要调试 使用 {:?} 其他使用 {}

    // debug 特征

    // let i = 3.1415926;
    // let s = String::from("hello");
    // let v = vec![1, 2, 3];
    // let p = Person {
    //     name: "sunface".to_string(),
    //     age: 18,
    // };
    // println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);

    let i = 3.1415926;
    let s = String::from("hello");
    // let v = vec![1, 2, 3];
    // let p = Person {
    //     name: "sunface".to_string(),
    //     age: 18,
    // };
    // println!("{}, {}, {}, {}", i, s, v, p);

    // v 和 p 会报错 因为虽然他们都实现了 debug 的特征 但是没有实现 display 特征 所以报错

    // 我们可以自定义 也可以使用 {:?} {:#?}

    // {:?} 和 {:#?} 的区别是 {:#?} 会格式化输出 都一样其实

    // {:?}
    // [1, 2, 3], Person { name: "sunface", age: 18 }

    // // {:#?}
    // [
    //     1,
    //     2,
    //     3,
    // ], Person {
    //     name: "sunface",
    // }

    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "大佬在上，请受我一拜，小弟姓名{}，年芳{}，家里无田又无车，生活苦哈哈",
                self.name, self.age
            )
        }
    }
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{}", p);
}
