use std::fs::File;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::net::IpAddr;
use std::result::Result;
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}

fn main() {
    let v = vec![1, 2, 3];
    // v[99];
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{home}");
    // let f = File::create("hello.txt");
    // println!("{:?}", f.unwrap().expect("Failed to create file"));
    let f = File::open("hello.txt");
    // println!("{:?}", w.unwrap().expect("Failed to open file"));

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    // if 分支控制

    let condition = true;
    let res = if condition == true { 55 } else { 66 };

    println!("{}", res);

    let n = 33;

    let aa = if n % 4 == 0 {
        999
    } else if n % 3 == 0 {
        888
    } else if n % 2 == 0 {
        777
    } else {
        666
    };

    println!("{}", aa);

    // 循环控制

    for i in 1..=99 {
        println!("{}", i);
    }

    let a = [10, 20, 30, 40, 50];

    for (i, v) in a.iter().enumerate() {
        println!("{}: {}", i, v);
    }

    for _ in 0..10 {
        // ...
    }

    let con = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for i in 0..con.len() {
        println!("{}", con[i]);
    }

    for i in con {
        println!("{}", i);
    }

    // for 循环 不需要任何条件限制 也不需要通过索引来访问 所以是最安全也是最常用的
    for i in 1..4 {
        if i == 2 {
            continue;
        };
        println!("{}", i);
    }

    // continue 跳过当前循环

    // break 跳出整个循环

    for i in 1..4 {
        let res = if i == 2 {
            break;
        };
        println!("{:?}", i);
    }
    let mut n = 2;
    while n <= 20 {
        println!("{}", n);
        n += 1;
    }
    println!("Hello, world!");
    println!("{n}");

    // while 循环 for
    println!("hello -------- baby");
    let re = [61, 54, 21, 112, 545, 66, 33];
    let mut i = 0;

    while i < re.len() {
        println!("{}", re[i]);
        i += 1;
    }

    for i in re.iter() {
        println!("{}", i);
    }

    // 不需要使用索引访问数组 安全简介 避免运行时的边界检查
    // 性能更高
    let mut ii = 6;
    loop {
        println!("again!");
        ii += 1;
        if ii == 10 {
            break;
        }
    }
    // loop 是一个表达式可以有返回值

    // break 可以当作return 带一个返回值

    let mut counter = 22;
    let res = loop {
        counter += 1;
        if (counter % 3) == 0 {
            break counter * 2;
        };
    };
    println!("{}", res);

    enum Direction {
        East,
        West,
        North,
        South,
    }
    let dire = Direction::East;
    match dire {
        Direction::East => println!("Im East! god"),
        Direction::West => println!("West"),
        Direction::North | Direction::South => println!("North or South"),
    }

    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter,
    // }

    // fn value_in_cents(coin: Coin) -> u16 {
    //     match coin {
    //         Coin::Penny => 111,
    //         Coin::Nickel => 222,
    //         Coin::Dime => 333,
    //         Coin::Quarter => 444,
    //     }
    // }

    // let resa = value_in_cents(Coin::Penny);
    // println!("{}", resa);

    fn value_in_cents(coin: Coin) -> u16 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));

    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRgb(i32, i32, i32),
    }

    let actions = vec![
        Action::Say("hello".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRgb(255, 0, 0),
    ];

    // println!("{:?}", actions);
    for action in actions {
        match action {
            Action::Say(s) => println!("{}", s),
            Action::MoveTo(x, y) => println!("Move to {}, {}", x, y),
            Action::ChangeColorRgb(r, g, b) => {
                println!("Change color to rgb({}, {}, {})", r, g, b)
            }
        }
    }

    // 必须穷尽匹配 不然会报错

    let some_u8_value = 100u8;
    println!("{}", some_u8_value);

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("other aaa 都没有啊"),
    }

    // if let 模式匹配 有时候可能只会有一种模式的值需要被处理 其他的值直接忽略的场景

    let v = Some(255u16);
    match v {
        Some(3) => println!("three"),
        _ => println!("other 255"),
    }

    if let Some(333) = v {
        println!("three");
    } else {
        println!("other 255");
    }

    enum MyEnum {
        Foo,
        Bar,
    }

    let e = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    // v.iter().filter(|x| x == MyEnum::Foo) 会报错 无法用x 跟一个枚举成员进行比较

    e.iter().filter(|x| matches!(x, MyEnum::Foo)).count();

    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));

    let f = File::open("hello1.txt");

    println!("{:?}", f);

    // let rew = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("problem opening the file: {:?}", error)
    //     },
    // };

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello1.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // 传播错误 读取文件用户名 进行返回

    // fn read_username_from_file() -> Result<String, io::Error> {
    //     // 打开文件，f是`Result<文件句柄,io::Error>`
    //     let f = File::open("hello.txt");

    //     let mut f = match f {
    //         // 打开文件成功，将file句柄赋值给f
    //         Ok(file) => file,
    //         // 打开文件失败，将错误返回(向上传播)
    //         Err(e) => return Err(e),
    //     };
    //     // 创建动态字符串s
    //     let mut s = String::new();
    //     // 从f文件句柄读取数据并写入s中
    //     match f.read_to_string(&mut s) {
    //         // 读取成功，返回Ok封装的字符串
    //         Ok(_) => Ok(s),
    //         // 将错误向上传播
    //         Err(e) => Err(e),
    //     };
    //     // ? 就是一个宏 跟match 一摸一样
    //     f.read_to_string(&mut s)?;
    // }

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }

    // ? 不仅仅可以用于 result 的传播 也可以用来当 option 的传播

    enum Option<T> {
        Some(T),
        None,
    }
    // 加了 ？ 号 如果get 的结果是 none 就直接返回 none 如果是 some 就把值给 v
    fn first(arr: &[i32]) -> Option<&i32> {
        let v = arr.get(0)?;
        Some(v)
    }

    fn first(arr: &[i32]) -> Option<&i32> {
        arr.get(0)
    }

    // 切记 ？操作服需要一个变量来承载正确的值 这个函数会返回 Some(&i32) 或者none
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
