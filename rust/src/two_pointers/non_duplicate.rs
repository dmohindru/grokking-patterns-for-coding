/*
Problem Statement
Given an array of sorted numbers, move all non-duplicate number instances at the beginning of the array in-place. The non-duplicate numbers should be sorted and you should not use any extra space so that the solution has constant space complexity i.e., .

Move all the unique number instances at the beginning of the array and after moving return the length of the subarray that has no duplicate in it.

Example 1:

Input: [2, 3, 3, 3, 6, 9, 9]
Output: 4
Explanation: The first four elements after moving element will be [2, 3, 6, 9].
Example 2:

Input: [2, 2, 2, 11]
Output: 2
Explanation: The first two elements after moving elements will be [2, 11].
Constraints:

1 <= nums.length <= 3 * 104
-100 <= nums[i] <= 100
nums is sorted in non-decreasing order.
*/
fn non_duplicate(input: &mut [i32]) -> usize {
    /*
    Time Complexity
    - While loop run max for input size of n. Therefore O(N)
    Space Complexity
    - Constant space required for this algorithm irrespective to the size of input. Therefore O(1)
     */
    let mut slow: usize = 0;
    let mut fast: usize = 1;
    let mut unique_nums: usize = 1;
    while fast <= input.len() - 1 {
        if input[slow] != input[fast] {
            slow += 1;
            unique_nums += 1;
        }
        input[slow] = input[fast];
        fast += 1;
    }

    unique_nums
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_valid_answer_for_non_duplicate_items() {
        let mut input1 = [2, 3, 3, 3, 6, 9, 9];
        let mut input2 = [2, 2, 2, 11];
        let mut input3 = [2, 2, 2, 2];
        let mut input4 = [1, 2, 3, 4, 5];

        assert_eq!(4, non_duplicate(&mut input1));
        assert_eq!(2, non_duplicate(&mut input2));
        assert_eq!(1, non_duplicate(&mut input3));
        assert_eq!(5, non_duplicate(&mut input4));
    }
}
