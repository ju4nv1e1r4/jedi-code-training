mod solution;
use crate::solution::Solution;

fn main() {
    let case_1 = String::from("()");
    let case_2 = String::from("()[]{}");
    let case_3 = String::from("(]");
    let case_4 = String::from("([])");
    let case_5 = String::from("([)]");
    let case_6 = String::from("(");


    let test_case_1 = Solution::is_valid(case_1);
    let test_case_2 = Solution::is_valid(case_2);
    let test_case_3 = Solution::is_valid(case_3);
    let test_case_4 = Solution::is_valid(case_4);
    let test_case_5 = Solution::is_valid(case_5);
    let test_case_6 = Solution::is_valid(case_6);

    println!("Test case 1 Result -> {:?}", test_case_1);
    println!("Test case 1 Result -> {:?}", test_case_2);
    println!("Test case 1 Result -> {:?}", test_case_3);
    println!("Test case 1 Result -> {:?}", test_case_4);
    println!("Test case 1 Result -> {:?}", test_case_5);
    println!("Test case 1 Result -> {:?}", test_case_6);
}
