/*
This example code counts the frequency of each number in the vector.
 */
use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn main() {
    // user inputs a vector of numbers
    println!("Enter a vector of numbers separated by spaces: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();

    //let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let mut result = logic(numbers);

    // we sort the result in descending order of the frequency
    result.sort_by(|a, b| b.1.cmp(&a.1));
    
    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
}
