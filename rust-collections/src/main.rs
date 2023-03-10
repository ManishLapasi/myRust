#[derive(Debug)]
enum spreadsheetCell {
    Int(i32),
    Float(f32),
    Text(String)
}


fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];

    v.push(1);
    v.push(2);
    v.push(3);

    let third_ele: &i32 = &v[2];
    // let fourth_ele2: &i32 = &v[3];           // this line will err on compile, since there is no fourth element!
    let fourth_ele: Option<&i32> = v.get(3);
    println!("accessing third element gives us {}", third_ele); 
    match fourth_ele {
        Some(fourth_ele) => println!("accessing fourth element gives us {fourth_ele}"),
        None => println!("there is no fourth element")
    }

    print_vec(&v);              // printing by passing it by reference to a function
    add_to_vec(&mut v, 50);         // adding x to element by passing a mutable reference to v
    print_vec(&v);

    let mut row: Vec<spreadsheetCell> = Vec::new();
    row.push(spreadsheetCell::Int(3));
    row.push(spreadsheetCell::Float(42.5));
    row.push(spreadsheetCell::Text(String::from("Hello there!")));
    print_row(&row);

}

fn print_row(v: &Vec<spreadsheetCell>) {
    for i in v {
        println!("{:#?}",i);
    }
}


fn print_vec(v: &Vec<i32>) {
    for i in v {
        println!("{i}");
    }
}

fn add_to_vec(v: &mut Vec<i32>, x: i32) {
    for i in v {
        *i += x;
    }
}