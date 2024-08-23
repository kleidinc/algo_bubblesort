fn bubblesort(collection: &mut Vec<i32>) -> &Vec<i32> {
    todo!();
}
fn main() {
    // 1. reate vector with random numbers
    let user_input = helpers::get::get_i32("Type a number and press enter");
    let vec = helpers::make_random_vec::make_random_vec(100, user_input);
    helpers::print_vec::print_vec(&vec, 50);

    // 2. sort the vector using bubblesort
    // 3. print the sorted version to the terminal
}

mod helpers;
