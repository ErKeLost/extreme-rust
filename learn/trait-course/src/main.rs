// // 特征的使用 例如 #[derive(Debug)] 用于打印结构体 打印结构体自动派生debug的特征
// // 还有例如add方法
// fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }
// // 通过 std::ops::Add 特征来限制 T 只有 T 实现了  Add 特征才能使用
// // 进行加法运算 毕竟不是所有类型都能相加

// // 定义特征 如果不同类型具有相同的行为 那么我们就可以定义一个特征 然后
// // 为不同的类型实现这个特征

// // 文件post 和 weibo 两种内容载体

// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

// // trait 关键字声明一个特征 summary 是特征名 在打括号中定义了该特征的所有方法
// // 特征只定义行为看起来是怎么样的 不定义行为具体是怎么样的 方法签名是一个 分号 不是 {}

// /**
//  *
//  * 为类型 post weibo 实现 summary 特征
//  */
// pub struct Post {
//     pub title: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("文章{}, 作者是{}", self.title, self.author)
//     }
// }

// pub struct Weibo {
//     pub username: String,
//     pub content: String,
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{}发表了微博{}", self.username, self.content)
//     }
// }

// pub struct Next {}

// impl Summary for Next {}

// // post 和 weibo 都实现了 summary 特征 所以可以使用 summary 特征的方法

// // 特征定义与实现的位置

// // 默认实现
// // 可以在特征中定义具有默认实现的方法 可以重载

// // trait 特征大放光彩的地方 作为函数参数
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// // 上面那个就是个语法糖 真正的书写 这是特定约束

// pub fn notify2<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// // 多重约束

// // 除了单个约束 指定多个约束
// pub fn notify3(item: &(impl Summary + Display)) {}
// pub fn notify4<T: Summary + Display>(item: &T) {}

// // where 约束
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {
// }
// fn main() {
//     let post = Post {
//         title: "Rust语言简介".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust棒极了!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "好像微博没Tweet好用".to_string(),
//     };

//     let next = Next {};

//     println!("{}", post.summarize());
//     println!("{}", weibo.summarize());
//     println!("{}", next.summarize());
// }

// // impl Summary for next {}

// // fn main() {
// //     println!("{}", next.summarize());
// // }

pub struct NewArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}发表了微博{}", self.username, self.content)
//     }
// }

impl Summary for Tweet {}

pub trait Summary {
    fn summarize(&self) -> String {
        // default impl
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;
}

// trait 作为参数

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 泛型语法
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("erkelost"),
        content: String::from("Rust棒极了"),
        reply: false,
        retweet: false,
    };

    let article = NewArticle {
        author: String::from("erkelost"),
        headline: String::from("Rust语言简介"),
        content: String::from("Rust棒极了"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());
}
