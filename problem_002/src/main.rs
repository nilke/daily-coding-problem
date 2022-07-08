// Good morning! Here's your coding interview problem for today.
// This problem was asked by Uber.
// Given an array of integers, return a new array such that each element at index i of the new array is the product of all the numbers in the original array except the one at i.
// For example, if our input was [1, 2, 3, 4, 5], the expected output would be [120, 60, 40, 30, 24]. If our input was [3, 2, 1], the expected output would be [2, 3, 6].
// Follow-up: what if you can't use division?

fn uber_array_modifier_nested_loop(int_array: Vec<i32>) -> Vec<i32> {

    let mut output_array: Vec<i32> = Vec::new();

    for (outer_index, _) in int_array.iter().enumerate() {
        let mut summation: i32 = 1;
        for (inner_index, number) in int_array.iter().enumerate() {
            if outer_index != inner_index {
                summation = summation * number;
            }
        }
        output_array.push(summation);
    }

    return output_array;
    
}

// fn uber_array_modifier_division(int_array: Vec<i32>) -> Vec<i32> {

//     let output_array: Vec<i32> = Vec::new();

//     for number in int_array.iter() {


//     }

//     return output_array;
    
// }

fn main() {
    let int_array: Vec<i32> = vec![1, 2, 3, 4, 5];
    let output_array: Vec<i32> = uber_array_modifier_nested_loop(int_array);
    print!("{:?}",output_array)
}