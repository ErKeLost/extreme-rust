// fn main() {
//     println!("Hello, world!");
//     // {
//     //     let r;                // ---------+-- 'a
//     //                           //          |
//     //     {                     //          |
//     //         let x = 5;        // -+-- 'b  |
//     //         r = &x;           //  |       |
//     //     }                     // -+       |
//     //                           //          |
//     //     println!("r: {}", r); //          |
//     // }                         // ---------+
//     // 悬垂指针和生命周期  r 引用了提前被释放的变量

//     // 函数中的生命周期

//     let string1 = String::from("adb");
//     let string2 = "abc";

//     let res = longest(string1.as_str(), string2);
//     println!("the longest string is {}", res);

//     fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//         if x.len() > y.len() {
//             x
//         } else {
//             y
//         }
//     };

//     // &i32 一个引用
//     // &'a i32 一个具有显式生命周期的引用
//     // &'a mut i32 一个具有显式生命周期的可变引用

//     // 和泛型一样 使用生命周期函数 需要 先声明 <'a>

//     // x, y 和 返回值至少活的和 'a 一样久 因为返回值要么是 x ， 要么是 y

//     let st = String::from("long lifecycle diff");

//     {
//         let st1 = String::from("xyz");
//         let a = longest(st.as_str(), st1.as_str());
//         println!("the longest string {}", a);
//     }

//     // 生命周期等于参数中生命周期最小的 result 的生命周期要等于 st1 的生命周期
//     // result 就要活的跟 st1 一样久

//     {
//         let string1 = String::from("long string is long");
//         let result;
//         {
//             let string2 = String::from("xyz");
//             result = longest(string1.as_str(), string2.as_str());
//         }
//         println!("The longest string is {}", result);
//     }

//     // 这段代码会报错 因为 result 的生命周期是 string1 和 string2 中最小的那个

//     // 深入思考生命周期标注

//     // 使用生命周期的方式 取决于函数的功能， 例如之前的 longest 函数，如果只返回第一个参数x

//     fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//         x
//     }
//     // 此例中 y没有被使用 y 的生命周期与x的返回值的生命周期无关 我们不必为了y标注生命周期 只需要标注

//     // x的参数和返回值就行

//     // 函数的返回值如果是一个引用类型 那么他的生命周期只会来源于 函数参数的生命周期 函数体重某个新建的引用生命周期

//     // 结构体中的生命周期

// }


fn main() {
    
}
