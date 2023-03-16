use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents) {
        println!("123, {line}");
    }

    // println!("With text:\n{contents}");

    Ok(())
}

impl Config {
    // fn new(args: &[String]) -> Config {
    //     if (args.len() < 3) {
    //         // panic!("not enough arguments");
    //         // 错误处理大法 返回一个 result
    //         return Err("not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();
    //     Ok(Config { query, file_path })
    // }
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // panic!("not enough arguments");
            // 错误处理大法 返回一个 result
            return Err("not enough arguments");
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }
    //     let ignore_case = env::var("IGNORE_CASE").is_ok();
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     Ok(Config {
    //         query,
    //         file_path,
    //         ignore_case,
    //     })
    // }
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 第一个参数是程序名，由于无需使用，因此这里直接空调用一次
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        // assert 用于比较两个值是否相等 如果不想等就打印出错误信息 是一个宏
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

// // in lib.rs
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     vec![]
// }

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }

//     results
// }

// 引入迭代器之后的 search 函数
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
