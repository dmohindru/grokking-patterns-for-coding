/*
Problem Statement
Given an array of unsorted numbers and a target number, find all unique quadruplets in it, whose sum is equal to the target number.

Example 1:

Input: [4, 1, 2, -1, 1, -3], target=1
Output: [-3, -1, 1, 4], [-3, 1, 1, 2]
Explanation: Both the quadruplets add up to the target.
Example 2:

Input: [2, 0, -1, 1, -2, 2], target=2
Output: [-2, 0, 2, 2], [-1, 0, 1, 2]
Explanation: Both the quadruplets add up to the target.
Constraints:

1 <= nums.length <= 200
-109 <= nums[i] <= 109
-109 <= target <= 109
*/
fn quadruple_sum_zero(mut input: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    /*
    Time Complexity
    - Sorting takes O(NLogN) time
    - This algorithm has three loop that each can run for input size n. There fore O(N^3)
    - Total time complexityL O(NLogN) + O(N^3). Considering only dominant terms
    - Total: O(N^3)

    Space Complexity
    - Sorting in rust is in place sorting, so no additional space required therefore O(1)
    - Space required store result in worst case is O(N^3)
    - Otherwise algorithm does uses constant amount of space, therefore O(1)
    - Total: O(1) + O(N^3) + O(1). Considering only dominant terms
    - Total: O(N^3)
    */
    input.sort();
    let mut result: Vec<Vec<i32>> = Vec::new();

    if input.len() < 4 {
        return result;
    }

    let mut i: usize = 0;

    while i < input.len() - 3 {
        if i > 0 && input[i] == input[i - 1] {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < input.len() - 2 {
            if j < i + 1 && input[j] == input[j - 1] {
                continue;
            }
            let mut left = j + 1;
            let mut right = input.len() - 1;
            let sum_target = target - (input[i] + input[j]);
            while left < right {
                let sum = input[left] + input[right];
                if sum > sum_target {
                    right -= 1;
                } else if sum < sum_target {
                    left += 1;
                } else {
                    result.push(vec![input[i], input[j], input[left], input[right]]);
                    while left < right && input[left] == input[left + 1] {
                        left += 1;
                    }
                    while left < right && input[right] == input[right - 1] {
                        right -= 1;
                    }

                    left += 1;
                    right -= 1;
                }
            }

            j += 1;
        }

        i += 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_unique_quadruple_sum_zero() {
        let test_cases = [
            (
                vec![4, 1, 2, -1, 1, -3],
                1,
                vec![vec![-3, -1, 1, 4], vec![-3, 1, 1, 2]],
            ),
            (
                vec![2, 0, -1, 1, -2, 2],
                2,
                vec![vec![-2, 0, 2, 2], vec![-1, 0, 1, 2]],
            ),
        ];

        for (input, target, expected_result) in test_cases {
            println!("Running test for {:?}", input);
            let result = quadruple_sum_zero(input, target);
            assert_eq!(expected_result.len(), result.len());
            assert_eq!(expected_result, normalize(result));
        }
    }

    fn normalize(mut data: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for inner in data.iter_mut() {
            inner.sort();
        }

        data.sort();

        data
    }
}
