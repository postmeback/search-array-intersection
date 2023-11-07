use std::{io, collections::HashSet};

fn main() {
   
   let mut first_array = String::new();

   let mut second_array  = String::new();

   println!("Enter the elements of first array");
   io::stdin().read_line(&mut first_array).expect("Unable to create first array");

   println!("Enter the elements of second array");
   io::stdin().read_line(&mut second_array).expect("Unable to create second array");

   let first_array1: Vec<i32> = first_array.split_whitespace().map(|s| s.parse().expect("Invalid Input")).collect();
   
   let second_array1: Vec<i32> = second_array.split_whitespace().map(|s| s.parse().expect("Invalid Input")).collect();

let result = intersection(first_array1, second_array1);

   println!("{:?}", result);

}

fn intersection(first_array1: Vec<i32>, second_array1: Vec<i32>) -> Vec<i32> {
    
    let set1: HashSet<i32> = first_array1.into_iter().collect();

    let set2: HashSet<i32> = second_array1.into_iter().collect();

    let common_elements: HashSet<i32> = set1.intersection(&set2).cloned().collect();

    let result: Vec<i32> = common_elements.into_iter().collect();

    return result;
}
