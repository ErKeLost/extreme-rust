fn main() {
    let temperature = 20;
    let weather = if temperature >= 25 { "hot" } else { "cool" };
    println!("The weather today is {}.", weather);

    // if 是表达式 表达式需要有返回值
    // 循环 三种循环方式 loop for while

    // loop 关键字会创建一个无限循环
    // let mut a = 1;
    // loop {
    //     a += 1;
    //     println!("again {}", a);
    //     if a > 10 {
    //         break;
    //     }
    // }
    let mut counter = 0;

    loop {
        counter += 1;
        if counter < 5 {
            continue;
        }
        println!("Hello, world!");
        if counter >= 5 {
            break;
        }
    }

    let target = 10;
    let mut sum = 0;
    let mut counter = 1;

    let result = loop {
        sum += counter;

        if sum >= target {
            break counter; // The value of counter will be returned from the loop as a result
        }
        println!("counter {}", counter);
        counter += 1;
    };

    println!(
        "The first number whose sum of all previous numbers is greater than or equal to {} is {}.",
        target, result
    );

    let numbers = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("The number is {}.", number);
    }

    for x in 1..=3 {
        println!("x: {}", x);
    }

    for x in 1..3 {
        println!("x: {}", x);
    }
}
