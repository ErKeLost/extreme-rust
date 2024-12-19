struct Point<T> {
    x: T,
    y: T,
}
fn print<T: std::fmt::Display>(p: Point<T>) {
    println!("Point {}, {}", p.x, p.y);
}

fn main() {
    let p = Point { x: 10, y: 20 };
    print(p);
    let p = Point { x: 10.2, y: 20.4 };
    print(p);

    // trait 中包含什么
    // 1. 包含关联函数，关联类型，常量

    struct Football;

    trait Sport {
        fn play(&self) {} // 注意这里一对花括号，就是trait的关联函数的默认实现
        fn play_mut(&mut self) {}
        fn play_own(self); // 注意这里是以分号结尾，就表示没有默认实现
        fn play_some() -> Self;
    }

    impl Sport for Football {
        fn play_own(self) {}
        fn play_some() -> Self {
            Self
        }
    }

    let mut f = Football;
    f.play(); // 方法在实例上调用
    f.play_mut();
    f.play_own();
    let _g = Football::play_some(); // 关联函数要在类型上调用
    let _g = <Football as Sport>::play_some(); // 注意这样也是可以的
}
