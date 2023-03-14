use std::env;
use std::process;

use minigrep::Config;
fn main() {
    // let args: Vec<String> = env::args().collect();
    // // dbg!(args);
    // let query = &args[1];
    // let file_path = &args[2];
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("Searching for {query}");
    // println!("In file {}", file_path);
    // println!("With text:\n{contents}");

    /**
         * 聚合配置变量
    前文提到，配置变量并不适合分散的到处都是，因此使用一个结构体来统一存放是非常好的选择
    ，这样修改后，后续的使用以及未来的代码维护都将更加简单明了。
         *
         */
    let args: Vec<String> = env::args().collect();

    // let config = parse_config(&args);

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // let contents = fs::read_to_string(config.file_path).expect("error");

    // println!("With text:\n{}", contents);

    // 还可以使用构造函数

    // let config = Config::new(&args);
    // 对 build 返回的 `Result` 进行处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // let contents = fs::read_to_string(config.file_path).expect("error");

    // println!("With text:\n{}", contents);

    // run(config);
    // 对 run 返回的 `Result` 进行处理
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        // --snip--
        println!("Application error: {e}");
        process::exit(1);
    }
}

// struct Config {
//     query: String,
//     file_path: String,
// }

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

// /**
//  *
//  * clone 的得与失

// 在上面的代码中，除了使用 clone ，还有其它办法来达成同样的目的，但 clone 无疑是最简单的方法：直接完整的复制目标数据，无需被所有权、借用等问题所困扰，但是它也有其缺点，那就是有一定的性能损耗。

// 因此是否使用 clone 更多是一种性能上的权衡，对于上面的使用而言，由于是配置的初始化，因此整个程序只需要执行一次，性能损耗几乎是可以忽略不计的。

// 总之，判断是否使用 clone:

// 是否严肃的项目，玩具项目直接用 clone 就行，简单不好吗？
// 要看所在的代码路径是否是热点路径(hot path)，例如执行次数较多的显然就是热点路径，热点路径就值得去使用性能更好的实现方式
//  *
//  */
//  下面我们试着来优化下，
// 通过构造函数来初始化一个 Config 实例，
// 而不是直接通过函数返回实例，典型的，标准库中的 String::new 函数就是一个范例。

// impl Config {
//     // fn new(args: &[String]) -> Config {
//     //     if (args.len() < 3) {
//     //         // panic!("not enough arguments");
//     //         // 错误处理大法 返回一个 result
//     //         return Err("not enough arguments");
//     //     }
//     //     let query = args[1].clone();
//     //     let file_path = args[2].clone();
//     //     Ok(Config { query, file_path })
//     // }
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             // panic!("not enough arguments");
//             // 错误处理大法 返回一个 result
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//         Ok(Config { query, file_path })
//     }

//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config { query, file_path })
//     }
// }

// // 分离主体逻辑

// // fn run(config: Config) {
// //     let contents =
// //         fs::read_to_string(config.file_path).expect("Should have been able to read the file");

// //     println!("With text:\n{contents}");
// // }

// // 错误处理逻辑
// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;

//     println!("With text:\n{contents}");

//     Ok(())
// }
