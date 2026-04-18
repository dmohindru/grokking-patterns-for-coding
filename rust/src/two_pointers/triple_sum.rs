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
    - Outer loop would run for full input size of N

    Space Complexity
    */
    input.sort();
    let mut i: usize = 0;
    let mut j: usize;
    let mut k: usize;
    let mut result: Vec<Vec<i32>> = Vec::new();
    while i < input.len() - 1 {
        j = i + 1;
        k = input.len() - 1;
        let target = 0 - input[i];
        while j < k {
            if input[j] + input[k] > target {
                k -= 1;
            } else if input[j] + input[k] < target {
                j += 1;
            } else {
                result.push(vec![input[i], input[j], input[k]]);
                j += 1;
                k -= 1;
            }
        }
        i += 1;
    }

    result
}

fn triple_sum_hash_map(mut input: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
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
    fn should_return_unique_triplets_for_hash_map_solution() {
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
            let result = triple_sum_hash_map(input);
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
