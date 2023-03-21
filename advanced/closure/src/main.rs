use std::thread;
use std::time::Duration;

fn muuuuu(inter: u32) -> u32 {
    println!("mu..");
    thread::sleep(Duration::from_secs(1));
    inter
}

fn workout(inter: u32, random_number: u32) {
    if inter < 24 {
        println!("活力满满 做{}俯卧撑", muuuuu(inter));
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            muuuuu(inter)
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!("昨天练过度了，今天干干有氧，跑步 {} 分钟!", muuuuu(inter));
    }
}

fn main() {
    let x = 1;
    let sum = |y| y + x;

    println!("sum: {}", sum(x));
    assert_eq!(3, sum(2));
    // 强度
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 3;

    // 开始健身
    workout(intensity, random_number);

    // rust 静态语言 所有的变量都有类型， 很多时候
    // 不需要显式的声明类型，rust 会自动推断
    // 但是函数不是 必须手动指明类型 如果当作api给用户使用
    // 用户就必须知道返回的制定类型
    // 但是闭包不需要 因为闭包不会作为api对外提供 因此它享受编译器的类型推导能力
    //

    let example_closure = |x| x;

    let res = example_closure(String::from("我是erkelost"));
    // let num = example_closure(8888);
    println!("res: {}", res);
    // println!("num: {}", num);

    // 这段代码会报错 因为一开始 推导出来了 string 又用了int 就不行了
    struct Cache<T, F>
    where
        T: Fn(F) -> F,
        F: Copy,
    {
        query: T,
        value: Option<F>,
    }

    impl<T, F> Cache<T, F>
    where
        T: Fn(F) -> F,
        F: Copy,
    {
        fn new(query: T) -> Cache<T, F> {
            Cache { query, value: None }
        }

        // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
        fn value(&mut self, arg: F) -> F {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.query)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut c = Cache::new(|x| x);
    println!("{}", c.value(32));
    let mut c = Cache::new(|x| x);
    println!("{}", c.value("abc"));

    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
    // 使用 Rust 的断言宏 assert!
    // 来验证调用闭包函数 equal_to_x 并传入参数 y 的结果是否为真（即 y 是否等于 x）
    // ，如果为真则程序继续执行，否则将会引发一个断言错误。

    fn fn_once<F>(func: F)
    where
        F: FnOnce(usize) -> bool + Copy,
    {
        println!("{}", func(3));
        println!("{}", func(4));
    }

    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());

    let mut s = String::new();

    let mut update_string = |str| s.push_str(str);
    update_string("hello");

    println!("{:?}", s);
}
