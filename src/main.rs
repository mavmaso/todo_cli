mod extra;

fn main() {
    let action = std::env::args().nth(1).expect("Please type a action");
    let item = std::env::args().nth(2).expect("Plase type an item");

    let mut todo = extra::Todo::new().expect("Failed to create db.txt");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("item saved"),
            Err(why) => println!("Error: {}", why),
        }
    }
}
