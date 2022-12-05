// fn main() {
//     // let mut x = 5;
//     // println!("The value of x is: {}", x);
//     // x = 6;
//     // println!("The value of x is: {}", x);
//     // let _r = 5;
//     // let _y = 10;
//     let (a, mut b): (bool, bool) = (true, false);
//     // a = true,不可变; b = false，可变
//     println!("a = {:?}, b = {:?}", a, b);

//     b = true;
//     assert_eq!(a, b);
// }
// fn main() {
//     let (a, mut b): (bool, bool) = (true, false);
//     // a = true,不可变; b = false，可变
//     println!("a = {:?}, b = {:?}", a, b);

//     b = true;
//     assert_eq!(a, b);
// }

// struct Struct {
//     e: i32,
// }

// fn main() {
//     let (a, b, c, d, e);

//     (a, b) = (1, 2);
//     // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Struct { e, .. } = Struct { e: 5 };

//     assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
// }

// fn main() {
//     let a: u8 = 255;
//     let b = a.wrapping_add(1);
//     println!("{}", b); // 19
//                        // 断言0.1 + 0.2与0.3相等
//                        // assert!(0.1 + 0.2 == 0.3);
//     let a: f32 = 0.1;
//     let b: f32 = 0.2;
//     println!("{}", a + b);
//     // 对于较长的数字，可以用_进行分割，提升可读性
//     let one_million: i64 = 1_000_000;
//     println!("{}", one_million.pow(2));
//     println!("{}", one_million);
// }

fn main() {
    // // 二进制为00000010
    // let a: i32 = 2;
    // // 二进制为00000011
    // let b: i32 = 3;

    // println!("(a & b) value is {}", a & b);

    // println!("(a | b) value is {}", a | b);

    // println!("(a ^ b) value is {}", a ^ b);

    // println!("(!b) value is {} ", !b);

    // println!("(a << b) value is {}", a << b);

    // println!("(a >> b) value is {}", a >> b);

    // let mut a = a;
    // // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    // a <<= b;
    // println!("(a << b) value is {}", a);

    for i in 'a'..='z' {
        println!("{}", i);
    }
    let x = '中';

    let aa = add(20, 40);
    println!("{}", aa);
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
}
fn add(i: i32, j: i32) -> i32 {
    i + j
}
