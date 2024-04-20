# Sorting Library

This Rust library offers a collection of efficient sorting algorithms, including Quick Sort, Merge Sort, Insertion Sort, and Selection Sort. It is designed to be flexible, allowing sorting of any object types as long as they adhere to the necessary trait bounds.

## Features

- **Quick Sort**: A divide-and-conquer algorithm for sorting that is efficient for large datasets.
- **Merge Sort**: A stable sorting algorithm that also uses the divide-and-conquer technique, suitable for sorting linked lists.
- **Insertion Sort**: A simple algorithm that works well for small datasets or arrays that are already partially sorted.
- **Selection Sort**: An in-place comparison sort that is noted for its simplicity, but typically outperformed by more complex algorithms.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sorting_lib = { git = "https://github.com/kunusaa/Assignment1-Blockchain2.git" }
```

## Usage
```rust
use sorting_lib::sorting::{quick_sort, merge_sort};
use std::cmp::Ordering;

fn main() {
    let mut numbers = vec![10, 5, 2, 3, 7];
    quick_sort(&mut numbers, &|a, &b| a.cmp(&b));
    println!("Quick Sorted: {:?}", numbers);

    let mut more_numbers = vec![10, 5, 2, 3, 7];
    merge_sort(&mut more_numbers, &|a, &b| a.cmp(&b));
    println!("Merge Sorted: {:?}", more_numbers);
}
```
