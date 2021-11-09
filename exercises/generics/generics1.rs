// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

struct Item<T>(T);

fn main() {
    let mut shopping_list = vec![];

    shopping_list.push(Item("milk"));
}
