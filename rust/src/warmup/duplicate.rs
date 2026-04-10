/*
Problem Statement
Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

Examples
Example 1:

Input: nums= [1, 2, 3, 4]
Output: false
Explanation: There are no duplicates in the given array.
Example 2:

Input: nums= [1, 2, 3, 1]
Output: true
Explanation: '1' is repeating.
Example 3:

Input: nums= [3, 2, 6, -1, 2, 1]
Output: true
Explanation: '2' is repeating.
*/
use std::collections::HashSet;

fn duplicate(nums: &[i32]) -> bool {
    let mut duplicates: HashSet<i32> = HashSet::new();
    for num in nums.iter() {
        let is_duplicate = duplicates.contains(num);
        if is_duplicate {
            return true;
        }
        duplicates.insert(*num);
    }

    false
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_return_true_for_duplicate_in_array() {
        let nums = [1, 2, 3, 1, 4];
        let is_duplicate = duplicate(&nums);
        assert_eq!(true, is_duplicate);
    }

    #[test]
    fn should_return_false_for_non_duplicate_in_array() {
        let nums = [1, 2, 3, 4];
        let is_duplicate = duplicate(&nums);
        assert_eq!(false, is_duplicate);
    }
}
