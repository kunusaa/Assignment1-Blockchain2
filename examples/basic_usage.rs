use sorting_lib::sorting::quick_sort;
use sorting_lib::sorting::selection_sort;
use sorting_lib::sorting::insertion_sort;   
use sorting_lib::sorting::merge_sort;

fn main() {
    let mut vec = vec![2, 7, 1, 19, 4, 5, 3, 11];
    quick_sort(&mut vec, &|a, &b| a.cmp(&b));
    println!("Sorted: {:?}", vec);

    let mut vec = vec![2, 7, 1, 19, 4, 5, 3, 11];
    selection_sort(&mut vec, &|a, &b| a.cmp(&b));
    println!("Sorted: {:?}", vec);

    let mut vec = vec![2, 7, 1, 19, 4, 5, 3, 11];
    insertion_sort(&mut vec, &|a, &b| a.cmp(&b));
    println!("Sorted: {:?}", vec);

    let mut vec = vec![2, 7, 1, 19, 4, 5, 3, 11];
    merge_sort(&mut vec, &|a, &b| a.cmp(&b));
    println!("Sorted: {:?}", vec);

}
