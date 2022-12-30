// use std::io;

// fn main() {
//     println!("Guess the number!");

//     println!("Please input your guess.");
//     // 声明可变与不可变的变量  变量的可变性 mut 代表可变
//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {}", guess);
// }

use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to");
        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_err) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("你选的数据大于随机数"),
            Ordering::Equal => {
                println!("你选的数据相等");
                break;
            }
        }
    }
}
