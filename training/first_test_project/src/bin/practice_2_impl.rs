struct People {
    name: String,
    age: i32,
}
impl People {
    fn display_people(&self) {
        println!("{:?}", self.name);
        println!("{:?}", self.age);
    }
}

fn main() {
    let test = People {
        name: "john merga".to_owned(),
        age: 34,
    };
    test.display_people();
    println!("{:?}", test.age);
    print_name(&test.name);
}

fn print_name(name: &str) {
    println!("{:?}", name);
}