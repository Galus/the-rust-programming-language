// fn ch4_1() {
//     let s = String::from("hello");

//     takes_ownership(s);
//     // println!("{}", s); // :)

//     let x = 5;

//     makes_copy(x);
// }

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

fn main() {
    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
