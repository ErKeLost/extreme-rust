fn main() {
    println!("Hello, world!");

    // rust 中定义变量叫 变量绑定 其他语言叫赋值 rust 叫绑定

    // rust 里面有一个最核心的原则就是 所有权 任何内存对象都是有主任的

    // rust 变量默认不可变 
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    // 使用下划线开头来忽略未使用的变量

    let _x = 5;
    let y = 10;

    /*
    --> src\main.rs:18:9
   |
18 |     let y = 10;
   |         ^ help: if this is intentional, prefix it with an underscore: `_y`
   |
   = note: `#[warn(unused_variables)]` on by default
    */
    // 变量的解构

    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
    // 常量不可变 且永远不可变 编译就确定值 必须声明类型
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // rust 允许声明相同的变量名 后面声明的变量会覆盖掉前面声明的变量 go 不行哈
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    //变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量
    // （在被遮蔽后，无法再访问到之前的同名变量），就可以重复的使用变量名字，而不用绞尽脑汁去想更多的名字。

    let mut spaces = "   ";
    spaces = spaces.len();
}
