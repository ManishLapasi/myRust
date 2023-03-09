mod front_of_house;
mod back_of_house;

pub fn eat_at_restaurant(toast: &str) {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();

    front_of_house::serving::take_order();
    front_of_house::serving::serve_order();
    front_of_house::serving::take_payment();

    back_of_house::fix_incorrect_order();
    let bf = back_of_house::order_breakfast(&toast);
    println!("Ordered breakfast with the following contents {:#?}", bf);

}