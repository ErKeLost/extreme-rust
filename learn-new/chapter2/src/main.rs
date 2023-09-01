use std::error::Error; // 使用 use 引入一个标准库的包，或者第三方的包

use clap::Parser; // clap 是一个 Rust 社区开发的命令行参数解析库
use reqwest::blocking::Client; // reqwest 是一个 Rust 社区开发的 HTTP 客户端库
use reqwest::header::HeaderMap;

#[derive(Parser)] // 这是宏，在高级特性章节中我们会学到宏的用法及原理
#[command(
    author,
    version,
    about = "Sends HTTP requests and prints detailed information"
)]
struct Cli {
    #[arg(short, long, help = "Target URL", required = true)]
    url: String,
}

/// Rust 程序入口
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    println!("Sending request to {}", cli.url);
    let response = send_request(&cli.url)?; // ? 是 Rust 中的错误传播语法糖，我们会在接下来的章节中学习

    print_response_details(response)?;

    Ok(())
}

/// 发起一个 HTTP 请求
fn send_request(url: &str) -> Result<reqwest::blocking::Response, Box<dyn Error>> {
    let client = Client::builder().build()?;
    let response = client.get(url).send()?;
    Ok(response)
}

/// 打印出 HTTP 响应的详细信息
fn print_response_details(response: reqwest::blocking::Response) -> Result<(), Box<dyn Error>> {
    println!("Status: {}", response.status());
    println!("Headers:");
    print_headers(response.headers());

    let body = response.text()?;
    // println!("Body:\n{}", body);

    Ok(())
}

/// 打印出 HTTP 响应头
fn print_headers(headers: &HeaderMap) {
    for (key, value) in headers.iter() {
        println!(" {}: {}", key, value.to_str().unwrap_or("[unprintable]"));
    }
    let z = 5; // 定义并初始化变量 z
    let result = {
        let mut temp = z * 3;
        temp - 1
    };
    println!("Result: {}", result); // 打印结果
}
