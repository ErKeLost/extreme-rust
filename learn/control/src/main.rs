// fn main() {
//     let a: i64 = 66666666666;
//     if i128::from(a) > 666666666666123123123 {
//         println!("我比999大");
//     } else {
//         println!("我比999小哦");
//     }
//     // test_if();
//     // test();
//     else_if()
// }

// fn test_if() {
//     let bb: i32 = if true { 200 } else { 500 };
//     println!("{}", bb);
// }

// fn test() {
//     let mut count = 0;
//     loop {
//         count += 1;
//         println!("{}", count);
//     }
// }

// fn else_if() {
//     let rmb = 7;
//     if rmb % 2 == 0 {
//         println!("我的是挣钱");
//     } else if rmb % 2 == 1 {
//         println!("你的有结余");
//     }
// }

// fn main() {
//     let mut count = 0;
//     'count: loop {
//         println!("count: {}", count);
//         let mut remaining = 10;
//         loop {
//             println!("remaining: {}", remaining);
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'count;
//             }
//             remaining -= 1
//         }
//         count += 1;
//     }
//     println!("end count: {}", count);
// }
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }

// fn main() {
//     let mut number: u32 = 0;
//     while number < 10 {
//         println!("6666");
//         number += 1;
//     }
//     println!("循环结束了")
// }

// 遍历元素中的合集

// fn main() {
//     const MAX: [i32; 6] = [66, 2, 3, 45, 21, 21];
//     let mut index = 0;
//     while index < MAX.len() {
//         println!("let index = {}", MAX[index]);
//         index += 1;
//     }
// }

// for

// fn main() {
//     let mut res = [123, 456, 2123, 787, 123, 123];
//     for element in res {
//         println!("res[{}] = ", element);
//     }
// }

fn main() {
    let mut res = [123, 456, 2123, 787, 123, 123];

    for number in (1..20).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
