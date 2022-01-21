//Arrays - fixed lists where elements are the same data types

use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    //Get single value
    println!("Single value: {}", numbers[0]);

    //Get array length
    println!("Array length: {}", numbers.len());

    //Arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    let mut _numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //Re-assign value
    _numbers[2] = 20;

    println!("Reassigned: {:?}", _numbers);

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
