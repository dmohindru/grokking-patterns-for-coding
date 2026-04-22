/*
Problem Statement
Given an array of unsorted numbers and a target number, find a triplet in the array whose sum is as close to the target number as possible, return the sum of the triplet.
If there are more than one such triplet, return the sum of the triplet with the smallest sum.

Example 1:

Input: [-1, 0, 2, 3], target=3
Output: 2
Explanation: The triplet [-1, 0, 3] has the sum '2' which is closest to the target.

There are two triplets with distance '1' from the target: [-1, 0, 3] & [-1, 2, 3].
Between these two triplets, the correct answer will be [-1, 0, 3] as it has a sum '2' which is less than the sum of the other triplet which is '4'.
This is because of the following requirement: 'If there are more than one such triplet, return the sum of the triplet with the smallest sum.'

Example 2:
Input: [-3, -1, 1, 2], target=1
Output: 0
Explanation: The triplet [-3, 1, 2] has the closest sum to the target.
Example 3:

Input: [1, 0, 1, 1], target=100
Output: 3
Explanation: The triplet [1, 1, 1] has the closest sum to the target.
Example 4:

Input: [0, 0, 1, 1, 2, 6], target=5
Output: 4
Explanation: There are two triplets with distance '1' from target: [1, 1, 2] & [0, 0, 6].
Between these two triplets, the correct answer will be [1, 1, 2] as it has a sum '4' which is less than the sum of the other triplet which is '6'.
This is because of the following requirement: 'If there are more than one such triplet, return the sum of the triplet with the smallest sum.'
Constraints:

3 <= arr.length <= 500
-1000 <= arr[i] <= 1000
-104 <= target <= 104

*/

/*
Approach to solve this or similar problem.
To run this algorithm efficiently answer lies in difference of target and distance rather than sum directly.
Few simple equation to keep in mind for the intuition
----------------------------------------------------
target = lowest_sum + distance_to_target -- 1
lowest_sum = target - distance_to_target -- 2
distance_to_target = target - lowest_sum -- 3
----------------------------------------------------

Other key observation in the difference equation like 2, 3 is the inverted behavior
for example equation
lowest_sum = target - distance_to_target
For lowest_sum to be a bigger number, distance_to_target should be small and vice versa
eg
target = 10
distance_to_target = 7
lowest_sum = 10 - 7 = 3 ---1

target = 10
distance_to_target = 5
lowest_sum = 5 ---2

observe in equation 2 lowest_sum (5) is greater than lowest_sum(3) in equation 2

So in this algorithm we need to exploit this behavior to improve the efficiency from O(n^3) brute force to O(n^2) using two pointer approach

key equation used in this algorithms
distance_to_target = target - sum
so if a sum is small, then distance_to_target becomes large and vice versa

sum = target - distance_to_target
so if distance_to_target is large sum is small and vice versa
*/

fn triple_sum_closest(mut input: Vec<i32>, target: i32) -> i32 {
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
    input.sort();
    let mut smallest_diff = i32::MAX;
    for i in 0..input.len() - 2 {
        let mut left = i + 1;
        let mut right = input.len() - 1;
        while left < right {
            let sum = input[i] + input[left] + input[right];
            let target_diff = target - sum;
            if target_diff == 0 {
                // difference of zero means we would the exact match
                // So return the sum which is a desired value
                return sum;
            } else if target_diff.abs() < smallest_diff.abs()
                || target_diff.abs() == smallest_diff.abs() && target_diff > smallest_diff
            {
                /*
                target_diff = target - sum
                if target_diff is +ve this means that sum < target -- situation 1
                if target_diff is -ve this means than sum > target -- situation 2
                from above this implies that sum in situation 1 < sum in situation 2
                even though its a same distance. And objective here is to select the triplet with least sum
                so we update smallest_diff with target_sum. Which will be used in return statement for calculating least sum as target - target_diff
                */
                smallest_diff = target_diff;
            }

            /*
            Because input is sorted and ideally
            target - sum should be zero or as close to zero
            however if
            target - sum is +ve. There for we need to increase sum so that it approaches zero, hence move left pointer to right
            other if
            target - sum is -ve. There for we need to decrease sum so that it approaches zero, hence move right pointer to left
            */
            if target_diff > 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    /*
    target = diff + sum
    sum = target - diff
    */
    target - smallest_diff
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_closest_sum() {
        let test_cases: [(Vec<i32>, i32, i32); 4] = [
            (vec![-1, 0, 2, 3], 3, 2),
            (vec![-3, -1, 1, 2], 1, 0),
            (vec![1, 0, 1, 1], 100, 3),
            (vec![0, 0, 1, 1, 2, 6], 5, 4),
        ];

        for (input, target, expected_result) in test_cases {
            println!("Testing for input={:?}, target={}", input, target);
            assert_eq!(expected_result, triple_sum_closest(input, target));
        }
    }
}
