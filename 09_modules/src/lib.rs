mod front_of_house;

mod back_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_watilist();
    crate::back_of_house::serving::prepare_food();

    let mut meal = crate::back_of_house::serving::Breakfast::summer("blueberries");
    meal.toast = String::from("wheat");

    println!(
        "{} and seasonal_fruit is: {}",
        meal.toast, meal.seasonal_fruit
    );
}
