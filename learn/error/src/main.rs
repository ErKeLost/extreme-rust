use std::fs::File;
use std::net::IpAddr;
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
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
