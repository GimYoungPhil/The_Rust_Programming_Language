pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            seat_at_table();
        }

        fn seat_at_table() {
            super::bula();
        }
    }

    fn bula() {}
}

use front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 절대경로
    // crate::front_of_house::hosting::add_to_waitlist();

    // 상대경로
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}

mod customer {
    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}
