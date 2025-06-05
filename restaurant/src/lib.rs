use std::fmt::Result;
use std::io::Result as IoResult;
use rand::{Rng,CryptoRng};

use std::io::{self, Write};
use std::cmp::*;

mod front_of_house;


// use front_of_house::hosting;
// use crate::front_of_house::hosting;
pub use self::front_of_house::hosting; // ew can use add_to_waitlist directly
//pub use crate::front_of_house::hosting; //same as above

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}


fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

mod back_of_house2 {
    pub struct Breakfast {
        pub toast: String,
         seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }   

        }
    }

}

pub fn eat_at_restaurant2() {
    let mut meal = back_of_house2::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // let meal2 = back_of_house2::Breakfast{
    //     toast: String::from("Wheat"),
    //     // seasonal_fruit: String::from("blueberries"), //we cannot create
    // };
}

mod back_of_house3 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn eat_at_restaurant3() {
    let order1 = back_of_house3::Appetizer::Soup;
    let order2 = back_of_house3::Appetizer::Salad;
}

fn function1()-> Result{
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}