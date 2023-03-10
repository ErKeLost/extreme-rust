fn main() {
    // rust 使用 impl 定义方法
    struct Circle {
        x: f64,
        y: f64,
        radius: f64,
    }
    impl Circle {
        // 初始化方法
        fn new(x: f64, y: f64, radius: f64) -> Circle {
            Circle {
                x: 0.666,
                y: y,
                radius: radius,
            }
        }

        fn area(self: &Self) -> f64 {
            println!("{}", self.x);
            std::f64::consts::PI * (self.radius * self.radius)
        }
    }

    let res = Circle {
        x: 0.5,
        y: 0.5,
        radius: 0.5,
    };

    let data = res.area();

    println!("{:?}", data);
    println!("{:?}", std::f64::consts::PI);

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
