/*
Problem Statement
Given an array of unsorted numbers, find all unique triplets in it that add up to zero.

Examples
Example 1
Input: [-3, 0, 1, 2, -1, 1, -2]
Output: [[-3, 1, 2], [-2, 0, 2], [-2, 1, 1], [-1, 0, 1]]
Explanation: There are four unique triplets whose sum is equal to zero.
Example 2
Input: [-5, 2, -1, -2, 3]
Output: [[-5, 2, 3], [-2, -1, 3]]
Explanation: There are two unique triplets whose sum is equal to zero.
Constraints:

3 <= arr.length <= 3000
-105 <= arr[i] <= 105
*/

fn triple_sum_two_pointer(mut input: Vec<i32>) -> Vec<Vec<i32>> {
    /*
    Time Complexity
    - Sorting would take O(NLogN) time
    - Outer loop would run for input size of N. Therefore O(N)
    - Inner loop would also run for input size of N in worst case scenario. Therefore O(N)
    - Total: O(NLogN) + O(N) * O(N) = O(NLogN) + O(N^2).
    - Ignoring non dominating terms. Final Time complexity O(N^2)

    Space Complexity
    - This algorithm uses same amount of space irrespective of input size. Therefore O(1)
    - Sorting requires O(N) additional space
    - To store the output triples. In worst case it can be O(N^2)
    - Therefore total: O(N^2)
    */
    input.sort();
    let mut result: Vec<Vec<i32>> = Vec::new();
    if input.len() < 3 {
        return result;
    }

    let mut i: usize = 0;
    while i + 2 < input.len() {
        if i > 0 && input[i] == input[i - 1] {
            i += 1;
            continue;
        }

        let mut j: usize = i + 1;
        let mut k: usize = input.len() - 1;
        let target = 0 - input[i];

        while j < k {
            let sum = input[j] + input[k];
            if sum > target {
                k -= 1;
            } else if sum < target {
                j += 1;
            } else {
                result.push(vec![input[i], input[j], input[k]]);

                while j < k && input[j] == input[j + 1] {
                    j += 1;
                }
                while j < k && input[k] == input[k - 1] {
                    k -= 1;
                }

                j += 1;
                k -= 1;
            }
        }

        i += 1;
    }

    result
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_return_unique_triplets_for_two_pointer_solution() {
        let test_cases = [
            (
                vec![-3, 0, 1, 2, -1, 1, -2],
                vec![
                    vec![-3, 1, 2],
                    vec![-2, 0, 2],
                    vec![-2, 1, 1],
                    vec![-1, 0, 1],
                ],
            ),
            (
                vec![-5, 2, -1, -2, 3],
                vec![vec![-5, 2, 3], vec![-2, -1, 3]],
            ),
        ];

        for (input, expected_result) in test_cases {
            println!("Running test for {:?}", input);
            let result = triple_sum_two_pointer(input);
            assert_eq!(expected_result.len(), result.len());
            assert_eq!(expected_result, normalize(result));
        }
    }

    #[test]
    fn should_skip_duplicate_triplets() {
        let duplicate_cases = [
            (vec![0, 0, 0, 0], vec![vec![0, 0, 0]]),
            (vec![-1, 0, 1, -1, 0, 1], vec![vec![-1, 0, 1]]),
            (vec![-2, 0, 2, 2, 0, -2], vec![vec![-2, 0, 2]]),
        ];

        for (input, expected) in duplicate_cases {
            let two_pointer = normalize(triple_sum_two_pointer(input.clone()));

            assert_eq!(expected, two_pointer);
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
