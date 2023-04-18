// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a hint.



fn main() {
    let mut shopping_list: Vec<&str> = Vec::new(); // The type needed to be &str here
    shopping_list.push("milk");
}
