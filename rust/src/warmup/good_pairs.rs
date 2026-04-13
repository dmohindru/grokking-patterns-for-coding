/*
Problem Statement
Given an array of integers nums, return the number of good pairs.

A pair (i, j) is called good if nums[i] == nums[j] and i < j.

Example 1:

Input: nums = [1,2,3,1,1,3]
Output: 4
Explanation: There are 4 good pairs, here are the indices: (0,3), (0,4), (3,4), (2,5).
Example 2:

Input: nums = [1,1,1,1]
Output: 6
Explanation: Each pair in the array is a 'good pair'.
Example 3:

Input:  nums = [1,2,3]
Output: 0
Explanation: No number is repeating.
Constraints:

1 <= nums.length <= 100
1 <= nums[i] <= 100

*/
use std::collections::HashMap;
fn good_pairs(input: &[i32]) -> i32 {
    /*
    Time Complexity
    - Loop in executed only once dependent on size of input array.
    - Total Complexity O(N)

    Space Complexity
    - It uses HashMap and its size can grow to input size N in worst case scenario where each element in input array is different.
    - Total Complexity O(N)
    */
    let mut seen_nums: HashMap<i32, i32> = HashMap::new();
    let mut good_pairs = 0;
    for num in input {
        match seen_nums.get(num) {
            Some(val) => {
                good_pairs += val + 1;
                seen_nums.insert(*num, val + 1);
            }
            None => {
                seen_nums.insert(*num, 0);
            }
        }
    }

    good_pairs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_good_pairs() {
        let nums = [1, 2, 3, 1, 1, 3];
        println!("Running tests for input={:?}", nums);
        assert_eq!(4, good_pairs(&nums));

        let nums = [1, 1, 1, 1];
        println!("Running tests for input={:?}", nums);
        assert_eq!(6, good_pairs(&nums));

        let nums = [1, 2, 3];
        println!("Running tests for input={:?}", nums);
        assert_eq!(0, good_pairs(&nums));
    }
}
