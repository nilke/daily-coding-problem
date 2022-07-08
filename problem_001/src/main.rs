// Good morning! Here's your coding interview problem for today.
// This problem was recently asked by Google.
// Given a list of numbers and a number k, return whether any two numbers from the list add up to k.
// For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.
// Bonus: Can you do this in one pass?

fn google_check_sum(numbers_to_compare: Vec<i32>, k: i32) -> bool {
    let mut is_match = false;
    let mut would_be_matches: Vec<i32> = Vec::new();

    for number in numbers_to_compare.iter() {
        if would_be_matches.contains(number) {
            is_match = true;
        }

        let would_be_match: i32 = k - number;
        would_be_matches.push(would_be_match);
    }

    return is_match;
}


fn main() {
    let list_of_numbers: Vec<i32> = vec![12, 15, 3, 7];
    let k: i32 = 17;
    let does_add_up = google_check_sum(list_of_numbers, k );
    
    match does_add_up {
        true => print!("Adds up!"),
        false => print!("Does not add up!")
    }
}