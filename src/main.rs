fn bubblesort(collection: &mut Vec<i32>) {
    // make a clone for immutable access
    let mut collection_clone = collection.clone();
    let mut switched;
    loop {
        switched = false;
        for (index, element) in collection_clone.iter().enumerate() {
            if index < collection_clone.len() - 1 {
                if element > &collection_clone[index + 1] {
                    collection.swap(index, index + 1);
                    switched = true;
                }
            }
        }
        collection_clone = collection.clone();

        if !switched {
            break;
        }
    }
}

fn main() {
    // 1. reate vector with random numbers
    let user_input = helpers::get::get_i32("Type a number and press enter");
    let mut vec = helpers::make_random_vec::make_random_vec(500, user_input);
    bubblesort(&mut vec);
    helpers::print_vec::print_vec(&vec, 50);
    // 2. sort the vector using bubblesort
    // 3. print the sorted version to the terminal
}

mod helpers;
