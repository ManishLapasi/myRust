pub mod hosting {
    pub fn add_to_waitlist() {
        println!("added to waitlist!");
    }

    pub fn seat_at_table() {
        println!("seated at table");
    }
}

pub mod serving {
    pub fn take_order() {
        println!("taking your order");
    }

    pub fn serve_order() {
        println!("serving your order");
    }

    pub fn take_payment() {
        println!("receiving your payment");
    }
}