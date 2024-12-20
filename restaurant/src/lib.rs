mod front_of_house; 

pub use crate::front_of_house::{hosting, serving};

fn deliver_order() {
    serving::serve();
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    #[derive(PartialEq)]
    pub enum Appetizer {
        Soup,
        Salad
    }



    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn get_seasonal_fruit(&self) -> &String {
            &self.seasonal_fruit
        }

        pub fn set_seasonal_fruit(&mut self, new_fruit: String) {
            self.seasonal_fruit = new_fruit;
        }
    }
    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
}
// use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    
    hosting::add_to_waitlist();

    // front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    println!("Received \"{}\" fruit", meal.get_seasonal_fruit());

    // Lets say we ran out of fruit and replaced it
    let new_fruit: String = String::from("berries");
    meal.set_seasonal_fruit(new_fruit);
    println!("Changed fruit to \"{}\"", meal.get_seasonal_fruit());

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    match order1 {
        back_of_house::Appetizer::Salad => println!(""),
        back_of_house::Appetizer::Soup => {
            match order2 {
                back_of_house::Appetizer::Salad |
                back_of_house::Appetizer::Soup => println!("")
            }
        }
    }
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
