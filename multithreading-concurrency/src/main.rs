// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handler = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     handle.join().unwrap();
//     handler.join().unwrap();
//     // 创建一个线程A
//     let new_thread = thread::spawn(move || {
//         // 再创建一个线程B
//         thread::spawn(move || loop {
//             println!("I am a new thread.");
//         })
//     });

//     // 等待新创建的线程执行完成
//     new_thread.join().unwrap();
//     println!("Child thread is finish!");

//     // 睡眠一段时间，看子线程创建的子线程是否还在运行
//     thread::sleep(Duration::from_millis(10000000));
// }

use std::sync::{Arc, Barrier};
use std::thread;
fn main() {
    // 线性屏障 可以让多个线程全部执行到某个点之后在继续执行之后的逻辑
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));
    for _ in 0..6 {
        let c = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("before wait");
            c.wait();
            println!("after wait");
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
