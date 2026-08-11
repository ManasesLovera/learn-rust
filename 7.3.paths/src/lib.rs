mod back_of_house;
mod math;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!(
        "I'd like {} toast please, also {}",
        meal.toast, meal.seasonal_fruit
    );
}

pub fn calculate() {
    let result = math::add(12, 12);
    println!("Result: {}", result);
}
