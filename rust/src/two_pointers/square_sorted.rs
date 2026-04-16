/*
Problem Statement
Given a sorted array, create a new array containing squares of all the numbers of the input array in the sorted order.

Example 1:

Input: [-2, -1, 0, 2, 3]
Output: [0, 1, 4, 4, 9]
Example 2:

Input: [-3, -1, 0, 1, 2]
Output: [0, 1, 1, 4, 9]
Constraints:

Input: [0, 1, 2, 3, 4]

1 <= arr.length <= 104
-104 <= arr[i] <= 104
arr is sorted in non-decreasing order.
*/

fn sorted_squared_array(input: &[i32]) -> Vec<i32> {
    /*
    Time Complexity
    - While loop run max for size of input N in worst case. Therefore O(N)
    Space Complexity
    - Algorithm does return a result as array of size same as input. Therefore O(N)
     */
    let mut result = Vec::from(input);
    let mut left = 0;
    let mut right = input.len() - 1;
    let mut index = right;
    while left <= right {
        let left_squared = input[left] * input[left];
        let right_squared = input[right] * input[right];
        if right_squared > left_squared {
            result[index] = right_squared;
            right -= 1;
        } else {
            result[index] = left_squared;
            left += 1;
        }
        // To protect against subtract underflow in Rust
        if index > 0 {
            index -= 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_right_sorted_array() {
        let input1 = [-2, -1, 0, 2, 3];
        let expected_answer1 = [0, 1, 4, 4, 9];
        assert_eq!(expected_answer1.to_vec(), sorted_squared_array(&input1));

        let input1 = [0, 1, 2, 2, 3];
        let expected_answer1 = [0, 1, 4, 4, 9];
        assert_eq!(expected_answer1.to_vec(), sorted_squared_array(&input1));
    }
}
