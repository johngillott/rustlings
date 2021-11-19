// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    println!("printing: {}", maybe_number.unwrap());
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers = vec![Some(0); 5];
    for iter in 0..5 {
        let number_to_add = { Some(((iter * 1235) + 2) / (4 * 16)) };

        numbers[iter as usize] = number_to_add;
    }
}
