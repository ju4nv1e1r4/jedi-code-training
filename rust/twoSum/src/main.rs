mod two_sum;
use two_sum::two_sum;

fn main() {
    let test_case_1 = vec![2, 7, 11, 15];
    let test_case_2  = vec![3, 2, 4];
    let test_case_3 = vec![3, 3];
    let target_1 = 9;
    let target_2 = 6;
    let target_3 = 6;
    println!("{:?}", two_sum(test_case_1, target_1));
    println!("{:?}", two_sum(test_case_2, target_2));
    println!("{:?}", two_sum(test_case_3, target_3));
}
