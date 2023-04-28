mod front_of_house {
    mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 支持绝对路径 和 相对路径
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat() {
    add_to_waitlist()
}
