// mod practice.1;

// // Import (via `use`) the `fmt` module to make it available.
// use std::fmt;

// // Define a structure for which `fmt::Display` will be implemented. This is
// // a tuple struct named `Structure` that contains an `i32`.
// struct Person<'a> {
//     name: &'a str,
//     age: u8
// }

// // To use the `{}` marker, the trait `fmt::Display` must be implemented
// // manually for the type.
// impl fmt::Display for Structure {
//     // This trait requires `fmt` with this exact signature.
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write strictly the first element into the supplied output
//         // stream: `f`. Returns `fmt::Result` which indicates whether the
//         // operation succeeded or failed. Note that `write!` uses syntax which
//         // is very similar to `println!`.
//         write!(f, "{}", self.0)
//     }
// }

// fn main (){
//     println!("{}", Structure(3))
// }
#![allow(dead_code)]

// use std::fmt
#[derive(Debug)]
enum Status {
    ACTIVE,
    INACTIVE,
}
// impl fmt::Display for Status {
//     fn fmt(&self, f: &mut fmt::Formatter)-> std::fmt::Result{
//         match self {
//             Status::ACTIVE => write!(f,"ACTIVE"),
//             Status::INACTIVE => write!(f, "INACTIVE"),

//         }
//     }
// }
struct People {
    name: String,
    age: u8,
}
fn main() {
    #[allow(dead_code)]
    // let person_1_status = Status::ACTIVE;

    match Status::ACTIVE {
        Status::ACTIVE => println!("Active"),
        Status::INACTIVE => println!("Inactive"),
    }
    // change Status::ACTIVE to string

    let sample_tuple = (32, Status::ACTIVE);
    let data = accept_tuple(&sample_tuple);
    let data2 = accept_tuple(&sample_tuple);
    println!("{data:?}");
    println!("{data2:?}");
}
fn accept_tuple(data: &(i32, Status)) -> &(i32, Status) {
    return data;
}
