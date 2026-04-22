/*
Problem Statement
Given an array arr of unsorted numbers and a target sum, count all triplets in it such that arr[i] + arr[j] + arr[k] < target where i, j, and k are three different indices. Write a function to return the count of such triplets.

Example 1:

Input: [-1, 0, 2, 3], target=3
Output: 2
Explanation: There are two triplets whose sum is less than the target: [-1, 0, 3], [-1, 0, 2]
Example 2:

Input: [-1, 4, 2, 1, 3], target=5
Output: 4
Explanation: There are four triplets whose sum is less than the target:
[-1, 1, 4], [-1, 1, 3], [-1, 1, 2], [-1, 2, 3]
Constraints:

n == arr.length
0 <= n <= 3500
-100 <= arr[i] <= 100
-100 <= target <= 100
*/

fn triple_sum_smallest(mut input: Vec<i32>, target: i32) -> i32 {
    /*
    Equation for this problem
    X + Y + Z < Target
    Y + X < Target - X
    */
    /*
    Time Complexity
    - Sorting input will have time complexity of O(NLogN)
    - Outer for will run for n-2 iteration for input size of n. Therefore O(n-2), which approx equals to O(n)
    - Inner loop would run for n-2 iteration for input size of n. Therefore O(n-2), which approx equals to O(n)
    - Total time complexity: O(NLogN + N * N). Ignoring less significant fields, therefore O(N^2)

    Space Complexity
    - Sorting input in rust is in place so space required is O(1)
    - Constant about of space is required for running the actual algorithm, hence O(1)
    - Total space required, O(1)
    */
    if input.len() < 3 {
        panic!("Invalid input");
    }
    let mut count = 0;
    input.sort();
    for i in 0..input.len() - 2 {
        let mut left: usize = i + 1;
        let mut right = input.len() - 1;
        while left < right {
            let sum = input[i] + input[left] + input[right];
            if sum < target {
                /*
                Key observation here is this line of code.
                Since input is sorted and monotonically increasing.
                All the pairs from left to right would work, keep left fixed
                e.g.
                input[i] + input[left] + input[left+1] < target
                input[i] + input[left] + input[left+2] < target
                ...
                input[i] + input[left] + input[right] < target
                So essentially give a total of (right-left) pairs
                */
                count += (right - left) as i32;
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "Invalid input")]
    fn should_panic_if_input_size_less_than_three() {
        triple_sum_smallest(vec![1, 2], 0);
    }

    #[test]
    fn should_return_smallest_triplet_sum() {
        let test_cases = [(vec![-1, 0, 2, 3], 3, 2), (vec![-1, 4, 2, 1, 3], 5, 4)];

        for (input, target, expected_response) in test_cases {
            println!("Testing for input: {:?}, target: {}", input, target);
            assert_eq!(expected_response, triple_sum_smallest(input, target))
        }
    }
}
