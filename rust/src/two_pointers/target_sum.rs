/*
Problem Statement
Given an array of numbers sorted in ascending order and a target sum, find a pair in the array whose sum is equal to the given target.

Write a function to return the indices of the two numbers (i.e. the pair) such that they add up to the given target. If no such pair exists return [-1, -1].

Example 1:

Input: [1, 2, 3, 4, 6], target=6
Output: [1, 3]
Explanation: The numbers at index 1 and 3 add up to 6: 2+4=6
Example 2:

Input: [2, 5, 9, 11], target=11
Output: [0, 2]
Explanation: The numbers at index 0 and 2 add up to 11: 2+9=11
Constraints:

2 <= arr.length <= 104
-109 <= arr[i] <= 109
-109 <= target <= 109
Only one valid answer exists.
*/

fn target_sum(input: &[i32], target: i32) -> (i32, i32) {
    /*
    Time complexity
    - While loop would run max of input size n in worst case scenario. Therefore O(N)

    Space complexity
    - Algorithm uses constant amount of memory irrespective of input size. Therefore O(1)

    Space complexity
     */
    let mut left = 0;
    let mut right = input.len() - 1;
    while left < right {
        let sum = input[left] + input[right];
        if sum == target {
            return (left as i32, right as i32);
        } else if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    }
    (-1, -1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_valid_pair_for_target_sum() {
        let test_cases: [(&[i32], i32, (i32, i32)); 4] = [
            (&[1, 2, 3, 4, 6], 6, (1, 3)),
            (&[2, 5, 9, 11], 11, (0, 2)),
            (&[1, 3, 8, 12], 11, (1, 2)),
            (&[-3, 1, 2, 4, 7, 10], 7, (0, 5)),
        ];

        for (input, target, expected) in test_cases {
            println!("Running test for input={:?}, target={}", input, target);
            assert_eq!(target_sum(input, target), expected);
        }
    }
}
