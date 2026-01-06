mod palindrome_number;
use palindrome_number::is_palindrome_number;

fn main() {
    let test_case_1 = 121;
    let test_case_2 = -121;
    let test_case_3 = 10;

    println!("Test Case 1: {}", is_palindrome_number(test_case_1));
    println!("Test Case 2: {}", is_palindrome_number(test_case_2));
    println!("Test Case 3: {}", is_palindrome_number(test_case_3));
}
