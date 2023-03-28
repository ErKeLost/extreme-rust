fn main() {
    // 栈内存从高位地址向下增长，栈内存是连续分配的 一般来说操作系统对栈内存大小
    // 都有限制 因此c 语言无法创建任意长度的数组， rust 中 main 线程栈大小 8m
    // 普通线程 2m 函数调用时会在其中创建一个临时栈空间调用结束后 Rust
    // 会让这个栈空间里的对象自动进入 Drop 流程，
    // 最后栈顶指针自动移动到上一个调用栈顶，无需程序员手动干预，因而栈内存申请和释放是非常高效的。

    // 堆内存从低位地址向上增长 堆内存通常只受物理内存限制 通常是不连续的，从性能角度看 栈往往比堆更高

    let b = foo("world");
    println!("{}", b);

    // 使用 box 把数据存在堆上
    let a = Box::new(3);
    println!("a = {}", a); // a = 3

    let b = *a + 99;
    println!("b {}", b);

    // a 持有的智能指针会在 作用域结束之后被释放掉 Box 实现了 Drop 特征

    // 在栈上创建一个 长度为 1000 的数组

    let arr = [0; 1000];

    // 由于 arr 分配在栈上 所以这里实际上是直接重新深拷贝了一份数据

    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    let arr3 = Box::new([0; 1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr4 = arr3;
    println!("arr4 {:#?}", arr4.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());

    // 将动态大小类型转变为 sized 固定大小类型

    // rust 需要在编译时 知道类型占用了多少空间 如果一种类型在编译时无法知道具体的大小

    // 那么被称为动态大小类型 DST
    // enum List {
    //   Cons(i32, List),
    //   Nil,
    // }
    // 递归类型 会报错 因为不知道大小

    // 可以通过把list 存储到堆上 并且用一个智能指针指向他

    // 就能完成 DST 到 Sized 的类型转变

    // enum List {
    //   Cons(i32, Box<List>),
    //   Nil,
    // }

    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];

    for e in elems {
        e.draw()
    }

    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
    //     使用 & 借用数组中的元素，否则会报所有权错误
    // 表达式不能隐式的解引用，因此必须使用 ** 做两次解引用，
    // 第一次将 &Box<i32> 类型转成 Box<i32>，
    // 第二次将 Box<i32> 转成 i32
}
fn foo(x: &str) -> String {
    let a = "Hello, ".to_string() + x;
    a
}

trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}
impl Draw for Button {
    fn draw(&self) {
        println!("这是屏幕上第{}号按钮", self.id)
    }
}

struct Select {
    id: u32,
}

impl Draw for Select {
    fn draw(&self) {
        println!("这个选择框贼难用{}", self.id)
    }
}
