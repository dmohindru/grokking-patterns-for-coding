/*
Problem Statement
Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.

You must not use any built-in exponent function or operator. For example, do not use pow(x, 0.5) in C++ or x ** 0.5 in Python.

Example 1:

Input: x = 8
Output: 2
Explanation: The square root of 8 is 2.8284, and since we need to return the floor of the square root (integer), hence we returned 2.
Example 2:

Input: x = 4
Output: 2
Explanation: The square root of 4 is 2.
Example 3:

Input: x = 2
Output: 1
Explanation: The square root of 2 is 1.414, and since we need to return the floor of the square root (integer), hence we returned 1.
Constraints:

0 <= x <= 231 - 1
*/

fn sqrt(input: u32) -> u32 {
    /*
    Time complexity
    - Loop uses binary search which as time complexity of O(log(N))
    - Loop is executed for almost half of the range of input N. Therefore O(log(N/2))
    - This simplifies to O(log(N))
    Space complexity
    - Algorithm uses fixed number of variable irrespective of input size
    - So space complexity: O(1)
    */
    if input == 0 || input == 1 {
        return input;
    }
    let mut left = 2;
    let mut right = input / 2;
    while left < right {
        let mid = left + (right - left) / 2;
        let num = mid * mid;
        if num == input {
            return mid;
        } else if num > input {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    right
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_right_floor_sqrt_value() {
        let cases: [(u32, u32); 7] = [(10, 3), (9, 3), (8, 2), (4, 2), (2, 1), (1, 1), (0, 0)];

        for (input, result) in cases {
            println!("Testing input={} for result={}", input, result);
            assert_eq!(result, sqrt(input));
        }
    }
}
