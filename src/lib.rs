mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        fn serve_order() {}
        pub fn take_payment() {}
    }
}

mod back_of_house {
    #[derive(Debug)]
    pub enum Toast {
        rye,
        wheat,
    }

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: Toast,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: Toast) -> Breakfast {
            Breakfast {
                toast,
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub fn cook_order() {}

    fn fix_incorrect_order() {
        super::back_of_house::Breakfast::summer(Toast::rye);
        cook_order();
    }
}

pub fn eat_at_restaurant() {
    back_of_house::cook_order();
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();

    let mut meal = back_of_house::Breakfast::summer(back_of_house::Toast::rye);

    println!("the breakfast is {:#?}", meal);
    println!("i don't like Rye, please change");

    meal.toast = back_of_house::Toast::wheat;
    println!("Okay that is your new meal: {:#?}", meal)
}