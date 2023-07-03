#[derive(Debug)]
struct People {
    name: String,
    age: u8,
    color: String,
}

impl People {
    fn print_name_and_color(&self) {
        match self.age {
            0..=18 => println!(
                "{} is a child, and hes favorite color is : {:?}",
                self.name, self.color
            ),
            19..=60 => println!(
                "{} is an adult and hes favorite color is : {:?}",
                self.name, self.color
            ),
            _ => println!(
                "{} is a senior and hes favorite color is : {:?}",
                self.name, self.color
            ),
        }
        match &self.name {
            name if name == "peter" => println!("{} is a cool name", name),
            _ => println!("{} is a weird name", self.name),
        }
    }
    fn print_people(&self) {
        println!("{:#?}", self);
    }
}
fn main() {
    let all_peoples = vec![
        People {
            name: String::from("john"),
            age: 34,
            color: String::from("green"),
        },
        People {
            name: String::from("peter"),
            age: 12,
            color: String::from("green"),
        },
        People {
            name: String::from("Betty"),
            age: 25,
            color: String::from("purple"),
        },
    ];
    for i in all_peoples {
        i.print_name_and_color();
        i.print_people()
    }

}
