#![allow(unused)]
#![allow(non_snake_case)]

pub mod rec_funcs;

fn main() {
    let track_id: String = String::from("2f9NLCoIaiIn7rZnH9mdir");
    rec_funcs::get_recomms(track_id);
}

