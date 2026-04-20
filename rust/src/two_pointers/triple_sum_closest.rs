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

Input: [-1, 0, 2, 3], target=3
Output: 2
Explanation: The triplet [-1, 0, 3] has the sum '2' which is closest to the target.
sum = -1 + 0 + 3 = 2,
target - sum = 3 - 2 = -1

target = diff + sum
sum = target - diff

targetSum = target - sum
if targetSum is +ve number that means sum is lesser than target
if targetSum is -ve number than means sum is greater than target
and in a situation where abs value of target - sum is equal and we need to select the smallest value of sum.
Then targetSum with a +ve value should be selected because this implies that sum would be the smallest of two.


ideally
target - sum should be zero or as close to zero
however if
target - sum is +ve. There for we need to increase sum so that it approaches zero
other if
target - sum is -ve. There for we need to decrease sum so that it approaches zero


-1 + 2 + 3 = 4

smallestDif = int_max_value
while i + 2 < n {
    left = i + 1
    right = n - 1
    while left < right {
        targetSum = input[i] + input[left] + input[right]
        targetDif = target - targetSum
        if targetDif == 0 {
            return targetSum
        }

        if targetDif < smallestDif.abs() {
            smallestDif = targetDif
        } else if targetDif == smallestDif.abs() {
            // need some clarifications here
        }

        // This is an inverted behaviour
        if targetDif > 0 {
        // increasing left would result in larger +ve targetSum and hence closer to zero targetDif
         left += 1
        } else {
        // Increasing right would result is
            right -= 1
         }
    }

    return target - smallestDif
}

*/

fn triple_sum_closest(mut input: Vec<i32>, target: i32) -> i32 {
    input.sort();
    let mut smallest_diff = i32::MAX;
    // let mut i: usize = 0;
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
                so we update smallest_diff with target_sum. Which will be used in return statement for calculating least sum as target - least_sum
                */
                smallest_diff = target_diff;
            }

            /*
            ideally
            target - sum should be zero or as close to zero
            however if
            target - sum is +ve. There for we need to increase sum so that it approaches zero
            other if
            target - sum is -ve. There for we need to decrease sum so that it approaches zero
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
