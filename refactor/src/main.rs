// // fn main() {
// //     // let mut x = 5;
// //     // println!("The value of x is: {}", x);
// //     // x = 6;
// //     // println!("The value of x is: {}", x);
// //     // let _r = 5;
// //     // let _y = 10;
// //     let (a, mut b): (bool, bool) = (true, false);
// //     // a = true,不可变; b = false，可变
// //     println!("a = {:?}, b = {:?}", a, b);

// //     b = true;
// //     assert_eq!(a, b);
// // }
// // fn main() {
// //     let (a, mut b): (bool, bool) = (true, false);
// //     // a = true,不可变; b = false，可变
// //     println!("a = {:?}, b = {:?}", a, b);

// //     b = true;
// //     assert_eq!(a, b);
// // }

// // struct Struct {
// //     e: i32,
// // }

// // fn main() {
// //     let (a, b, c, d, e);

// //     (a, b) = (1, 2);
// //     // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
// //     [c, .., d, _] = [1, 2, 3, 4, 5];
// //     Struct { e, .. } = Struct { e: 5 };

// //     assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
// // }

// // fn main() {
// //     let a: u8 = 255;
// //     let b = a.wrapping_add(1);
// //     println!("{}", b); // 19
// //                        // 断言0.1 + 0.2与0.3相等
// //                        // assert!(0.1 + 0.2 == 0.3);
// //     let a: f32 = 0.1;
// //     let b: f32 = 0.2;
// //     println!("{}", a + b);
// //     // 对于较长的数字，可以用_进行分割，提升可读性
// //     let one_million: i64 = 1_000_000;
// //     println!("{}", one_million.pow(2));
// //     println!("{}", one_million);
// // }

// fn main() {
//     // // 二进制为00000010
//     // let a: i32 = 2;
//     // // 二进制为00000011
//     // let b: i32 = 3;

//     // println!("(a & b) value is {}", a & b);

//     // println!("(a | b) value is {}", a | b);

//     // println!("(a ^ b) value is {}", a ^ b);

//     // println!("(!b) value is {} ", !b);

//     // println!("(a << b) value is {}", a << b);

//     // println!("(a >> b) value is {}", a >> b);

//     // let mut a = a;
//     // // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
//     // a <<= b;
//     // println!("(a << b) value is {}", a);

//     for i in 'a'..='z' {
//         println!("{}", i);
//     }
//     let x = '中';

//     let aa = add(20, 40);
//     println!("{}", aa);
//     println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
// }
// fn add(i: i32, j: i32) -> i32 {
//     i + j
// }

// 不可变引用
fn main() {
    let x = 5.5;
    let y = x;
    println!("{} {}", x, y);
    let mut s1 = String::from("woshinidie");
    let len = calculate_length(&mut s1);
    println!("The length of {} is {}.", s1, len)
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str("64564564564");
    s.len()
}

// 可变引用只能存在一个
// let mut s = String::from("hello");

// let r1 = &mut s;
// let r2 = &mut s;

// println!("{}, {}", r1, r2);


// 借用规则总结

// 同一时刻，你只能要么拥有一个可变引用 或者多个不可变引用

// 垂悬引用 就是垂悬指针

// 就是如果 指针指向了某个值之后，一个值已经被释放 ，但是指针还是依然存在，但是她的指针还是指向了堆中的一块内存
// 就是值已经被释放了 这块内存中已经没有任何东西了 或者已经被别的变量使用了 就没了


// String 最重要 的 数据类型

// 切片 对于字符串而言 切片就是对string类型中某一部分的引用

// let s = String::from("hello World");
// let hello = &s[0..4];
// let world = &s[6..11];


// // 如果想从索引0开始
// // 可以使用

// let s = String::from("hello World");
// let hello = &s[..4];
// let world = &s[..11];

// // 截取完整的string 切片

// let w = String::from("hello");
// let leng = w.len();
// let slice = &w[0..len]
// let slice = &w[..]


// 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，
// 也就是 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节，下面的代码就会崩溃：



//  let s = "中国人";
//  let a = &s[0..2];
//  println!("{}",a);
// 因为我们只取 s 字符串的前两个字节，但是本例中每个汉字占用三个字节，因此没有落在边界处，
// 也就是连 中 字都取不完整，此时程序会直接崩溃退出，如果改成 &s[0..3]，则可以正常通过编译
// 。 因此，当你需要对字符串做切片索引操作时，需要格外小心这一点, 关于该如何操作 UTF-8 字符串，参见这里

// 因为切片是对集合的部分引用 因此不仅仅字符串有切片 其他类型集合也有切片 例如数组
let a = [1,2,3,4,5,6,7,8,9,10];
let slice = a[1..3];
