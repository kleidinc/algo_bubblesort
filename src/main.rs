use is_sorted::IsSorted;
use std::time::{Duration, Instant};

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

fn check_sorted(collection: &mut Vec<i32>) {
    IsSorted::is_sorted(&mut collection.iter());
    println!("The vector is sorted!");
}

fn main() {
    // 1. reate vector with random numbers
    let num_items = helpers::get::get_i32("Amount of elements in the vec ");
    let max = helpers::get::get_i32("Max ");
    let mut vec = helpers::make_random_vec::make_random_vec(num_items, max);

    let start_timer = Instant::now();
    bubblesort(&mut vec);
    let duration = start_timer.elapsed();
    println!(
        "The sorting took {:?} milli-seconds !",
        duration.as_millis()
    );

    let max_print = helpers::get::get_i32("How many items to print ");
    helpers::print_vec::print_vec(&vec, max_print);

    check_sorted(&mut vec);
}

mod helpers;
