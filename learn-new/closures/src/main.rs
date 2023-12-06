fn main() {
    // 定义一个闭包并赋值给变量 add
    let add = |x, y| x + y;

    // 调用闭包
    let result = add(2, 3);
    println!("Result: {}", result);

    let outside_variable = 100;
    let closure = || {
        println!("This is a closure");
        println!("Outside variable: {}", outside_variable);
    };
    closure();

    let outside_variable2 = String::from("Hello");

    // 定义一个闭包，获取外部变量所有权
    let closure2 = move || {
        println!("Outside variable2: {}", outside_variable2);
    };
    closure2();
}
