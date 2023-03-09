struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

#[derive(Debug)]                    // this is to enable pretty printing
struct Rectangle {
    width: f32,
    height: f32
}

impl Rectangle {
    fn square(val: f32) -> Self {
        Self {
            width: val,
            height: val
        }
    }

    fn area(&self) -> f32 {
        self.width*self.height
    }

    fn can_hold(&self, r: &Rectangle) -> bool {
        return self.width > r.width && self.height > r.height;
    }
}


fn main() {
    let mut u1 = User {
        active: true,
        username: String::from("Manish"),
        email: String::from("some@none"),
        sign_in_count: 0
    };

    u1.email = String::from("none@some");

    let u2 = User {
        username: String::from("Mani"),
        email: String::from("some@some"),
        ..u1
    };

    for u in [u1,u2] {
        println!("user is {}, {}, {}, {}", u.active, u.username, u.email, u.sign_in_count);
    }

    let (p,c) = (Point(0,0,0), Color(0,0,0));
    println!("{},{},{}",p.0,p.1,p.2);
    println!("{},{},{}",c.0,c.1,c.2);

    let r = Rectangle {
        width: 2.0,
        height: 11.0
    };
    println!("rectangle is {:?}", r);           // print structs
    println!("rectangle is {:#?}", r);          // pretty print structs
    println!("{}",area_rect(&r));               // compute area using external function

    dbg!(&r);                                   // yet another way of printing

    println!("Area of r is {}", r.area());      // compue area using method defined for struct

    let r2 = Rectangle {
        width: 1.0,
        height: 5.0
    };
    println!("r can hold r2: {}",r.can_hold(&r2));
    println!("r2 can hold r: {}",r2.can_hold(&r));

    let s = Rectangle::square(5.0);
    println!("square area: {}", s.area());

}

fn area_rect(r: &Rectangle) -> f32 {
    return r.width*r.height;
}