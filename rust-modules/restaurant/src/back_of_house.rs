#[derive(Debug)]
pub struct Breakfast {
    pub toast: String,
    fruit: String
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast{
        Breakfast {
            toast: String::from(toast),
            fruit: String::from("peaches")
        }
    }
}

pub fn fix_incorrect_order() {
    cook_order();
    deliver_order();
}

pub fn cook_order(){
    println!("cooking order!");
}

pub fn order_breakfast(toast: &str) -> Breakfast {
    Breakfast::summer(toast)
}

fn deliver_order() {
    println!("order is being delivered!");
}
