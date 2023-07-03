// #[derive(Debug)]
struct Grocery {
    quantity: i32,
    id: i32,
}

fn main() {
    let small_grocery: Grocery = Grocery {
        quantity: 42,
        id: 29,
    };
    display_id(&small_grocery);
    display_quantity(&small_grocery)
}

fn display_quantity(grocery: &Grocery) {
    println!("{:?}", grocery.quantity)
}
fn display_id(grocery: &Grocery) {
    println!("{:?}", grocery.id);
}
