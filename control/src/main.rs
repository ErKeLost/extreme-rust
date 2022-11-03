fn main() {
    let a: i64 = 66666666666;
    if i128::from(a) > 666666666666123123123 {
        println!("我比999大");
    } else {
        println!("我比999小哦");
    }
    test_if();
    test();
}

fn test_if() {
    let bb: i32 = if true { 200 } else { 500 };
    println!("{}", bb);
}

fn test() {
    let mut count = 0;
    loop {
        count += 1;
        println!("{}", count);
    }
}
