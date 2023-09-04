fn main() {
    println!("Hello, world!");
    // 初始化枚举值
    let _ = FruitBox::Apple;
    let _ = FruitBox::Pear;
}
enum fruit {
    apple,
    banana,
    orange,
}

struct FruitBox {
    pub contains_apple: bool,
    pub contains_pear: bool,
}

