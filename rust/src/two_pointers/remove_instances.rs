/*
Problem 1: Given an unsorted array of numbers and a target ‘key’, remove all instances of ‘key’ in-place and return the new length of the array.

Example 1:

Input: [3, 2, 3, 6, 3, 10, 9, 3], Key=3
Output: 4
Explanation: The first four elements after removing every 'Key' will be [2, 6, 10, 9].
Example 2:

Input: [2, 11, 2, 2, 1], Key=2
Output: 2
Explanation: The first two elements after removing every 'Key' will be [11, 1].
*/

fn remove_instances(input: &mut [i32], key: i32) -> usize {
    /*
    Time Complexity
    - While loop run max for input size of n. Hence O(N)
    Space Complexity
    - Constant amount of space is used irrespective of size of input. Hence O(1)
     */
    let mut slow: usize = 0;
    let mut fast: usize = 0;

    while fast <= input.len() - 1 {
        if input[fast] != key {
            input[slow] = input[fast];
            slow += 1;
        }
        fast += 1;
    }
    slow
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_right_answer() {
        let mut input1 = [3, 2, 3, 6, 3, 10, 9, 3];
        let mut input2 = [2, 11, 2, 2, 1];
        let mut input3 = [2, 2, 2, 2, 2];

        assert_eq!(4, remove_instances(&mut input1, 3));
        assert_eq!(2, remove_instances(&mut input2, 2));
        assert_eq!(5, remove_instances(&mut input2, 3));
        assert_eq!(0, remove_instances(&mut input3, 2));
    }
}
