mod solution;
use solution::Solution;

fn main() {
    let case1 = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    let case2 = vec![
        String::from("dog"),
        String::from("racecar"),
        String::from("car"),
    ];

    let case3 = vec![
        String::from("abb"),
        String::from("abc"),
    ];

    let result_case1 = Solution::longest_common_prefix(case1);
    let result_case2 = Solution::longest_common_prefix(case2);
    let result_case3 = Solution::longest_common_prefix(case3);

    println!("{}", result_case1);
    println!("{}", result_case2);
    println!("{}", result_case3);

    println!("==== V2 of algorithm ====");

    let case4 = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];

    let case5 = vec![
        String::from("dog"),
        String::from("racecar"),
        String::from("car"),
    ];

    let case6 = vec![
        String::from("abb"),
        String::from("abc"),
    ];

    let result_v2_case4 = Solution::longest_common_prefix_v2(case4);
    let result_v2_case5 = Solution::longest_common_prefix_v2(case5);
    let result_v2_case6 = Solution::longest_common_prefix_v2(case6);

    println!("{}", result_v2_case4);
    println!("{}", result_v2_case5);
    println!("{}", result_v2_case6);
}
