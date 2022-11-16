mod todos;

use todos::Todo;

fn main() {
    let action = std::env::args().nth(1).expect("Please type a action");
    let item = std::env::args().nth(2).expect("Plase type an item");

    let todo = Todo::new().expect("Failed to create db");

    match action.as_str() {
        "add" => add_action(todo, item),
        "complete" => complete_action(todo, &item),
        _ => (),
    }
}

fn add_action(mut todo: Todo, item: String) {
    todo.insert(item);
    save_in(todo, "item saved")
}

fn complete_action(mut todo: Todo, item: &String) {
    match todo.complete(&item) {
        None => println!("item: '{}' not found", item),
        Some(_) => save_in(todo, "item completed"),
    }
}

fn save_in(todo: Todo, msg: &str) {
    match todo.save() {
        Ok(_) => println!("{}", msg),
        Err(why) => println!("Error: {}", why),
    }
}
