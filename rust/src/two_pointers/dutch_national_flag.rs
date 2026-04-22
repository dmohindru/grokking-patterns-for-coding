/*
Problem Statement
Given an array containing 0s, 1s and 2s, sort the array in-place. You should treat numbers of the array as objects, hence, we can’t count 0s, 1s, and 2s to recreate the array.

The flag of the Netherlands consists of three colors: red, white and blue; and since our input array also consists of three different numbers that is why it is called Dutch National Flag problem.

Examples
Example 1
Input: arr = [1, 0, 2, 1, 0]
Output: [0, 0, 1, 1, 2]
Explanation:
All 0s are moved to the front, 1s in the middle, and 2s at the end.
The relative order within each group doesn't matter.
Example 2
Input: arr= [2, 2, 0, 1, 2, 0]
Output: [0, 0, 1, 2, 2, 2]
Explanation:
All 0s come first, followed by the 1, and then all 2s at the end.
Sorting is done in-place without using extra space or counting.
Constraints:

n == arr.length
1 <= n <= 300
arr[i] is either 0, 1, or 2.
*/

fn dutch_national_flag_problem(mut input: Vec<i32>) -> Vec<i32> {
    /*
    Time complexity
    - While loop run max for input size n. Hence O(N)
    - Total time complexity. O(N)

    Space complexity
    - Algorithm requires constant space irrespective of input size N. Hence O(1)
    - Total space complexity: O(1)
    */
    let mut i: usize = 0;
    let mut left: usize = 0;
    let mut right = input.len() - 1;

    while i <= right {
        let num = input[i];
        if num == 0 {
            input.swap(i, left);
            i += 1;
            left += 1;
        } else if num == 2 {
            input.swap(i, right);
            // Rust specific fix
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            // num be be i
            i += 1;
        }
    }
    input
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_sorted_result() {
        let test_cases = [
            (vec![1, 0, 2, 1, 0], vec![0, 0, 1, 1, 2]),
            (vec![2, 2, 0, 1, 2, 0], vec![0, 0, 1, 2, 2, 2]),
            (vec![0, 0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0, 0]),
            (vec![1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1]),
            (vec![2, 2, 2, 2, 2, 2], vec![2, 2, 2, 2, 2, 2]), // This test is failing
            (vec![2, 1, 1, 0, 0, 0], vec![0, 0, 0, 1, 1, 2]),
        ];

        for (input, expected_output) in test_cases {
            println!("Testing for input={:?}", input);
            assert_eq!(expected_output, dutch_national_flag_problem(input));
        }
    }
}
