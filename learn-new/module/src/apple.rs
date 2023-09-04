pub fn eat_apple() {
    println!("I eat apple");
}

pub mod fruit {
    pub mod apple {
        pub fn eat_apple() {
            println!("I eat apple");
        }
    }

    pub mod pear {
        pub fn eat_pear() {
            println!("I eat pear");
        }
    }

    pub mod orange {
        pub fn eat_orange() {
            println!("I eat orange");
        }
    }
}
