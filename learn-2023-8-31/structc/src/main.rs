struct User {
    x: i8,
    y: i8,
}

fn main() {
    println!("Hello, world!");
    let user = User { x: 1, y: 2 };
    let res = create_user(user.x, user.y);
    println!("res: {:?}", res.x);
    struct Color(u8, u8, u8);
    struct Marker;
    let red = Color(255, 0, 0);
    let marker = Marker;
    // 单元结构体可以没有字段 也可以没有名称 看上面内个

    // 函数与方法的区别
    // 方法是定义在结构体上下文中的 第一个参数永远是 self 表示结构体示例
}
fn create_user(x: i8, y: i8) -> User {
    User { x: x, y: y }
}
