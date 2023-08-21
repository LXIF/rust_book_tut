mod front_of_house;

pub use crate::front_of_house::hosting; //re-exporting hosting, means hosting can be called at the root level

mod back_of_house {

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

mod customer {
    //bring hosting into scope
    // use crate::front_of_house::hosting;

    // use crate::back_of_house;
    // use crate::front_of_house;

    // use crate::back_of_house::Appetizer as Appetizer_alias; //renaming appetizer in this scope

    // use crate::{
    //     back_of_house, back_of_house::Appetizer, back_of_house::Appetizer as Appetizer_alias,
    //     front_of_house, front_of_house::hosting,
    // }; //bring both into scope
    use crate::{
        back_of_house::{self, Appetizer, Appetizer as Appetizer_alias},
        front_of_house::{self, hosting},
    }; //nest more!

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();

        //can use this after using use up above
        hosting::add_to_waitlist();

        //Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
        let order3 = Appetizer_alias::Salad;
    }
}
