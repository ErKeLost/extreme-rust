fn main() {
    let mut person = Person {
        name: String::from("Alice"),
        age: 99,
    };

    person.greet();

    person.change_name(String::from("Bob"));
    person.greet();

    let name = person.get_name_and_consumed();
    println!("The person's name was: {}", name);

    // println!("{}", person)
    let sq = Person::new(String::from("erkelost"), 3);

    println!("{:?}", sq);
}
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn change_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_name_and_consumed(self) -> String {
        self.name
    }

    // 除了可以定义方法还可以定义函数 就比如创建结构体的构造方法

    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
}
