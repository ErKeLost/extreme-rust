// fn first_word(s: &String) -> String {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.to_string()
// }

// fn main() {
//     let mut str = String::from("hello world");
//     let res = first_word(&str);
//     println!("{}", res);
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }
// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s); // word 的值为 5
//     println!("{}", word);
//     println!("{}", s);
//     s.clear(); // 这清空了字符串，使其等于 ""
//     println!("{}", word);
//     println!("{}", s);
//     // word 在此处的值仍然是 5，
//     // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
// }

// slice
// fn main() {
//     let s = String::from("hello world");

//     let hello = &s[0..4];
//     let world = &s[7..11];
//     println!("{}", hello);
//     println!("{}", world);

// }

// 切片索引从0开始 可以不写 0 直接 ..11
// fn main() {
//     let s = String::from("hello");

//     let slice = &s[0..2];
//     let slice = &s[..2];
//     println!("{}", slice);
// }

fn main() {
    let s = String::from("hello");

    let len = s.len();
    let a: usize = 1;

    // let slice = &s[a..len];
    // 也可以 不写index 如果要获取全部
    let slice = &s[..];
    let slice = &s;
    println!("{}", slice);
}
