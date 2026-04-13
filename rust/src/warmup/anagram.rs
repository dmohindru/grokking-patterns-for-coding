/*
Problem Statement
Given two strings s and t, return true if t is an anagram of s, and false otherwise.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, using all the original letters exactly once.

Example 1:

Input: s = "listen", t = "silent"
Output: true
Example 2:

Input: s = "rat", t = "car"
Output: false
Example 3:

Input: s = "hello", t = "world"
Output: false
Constraints:

1 <= s.length, t.length <= 5 * 104
s and t consist of lowercase English letters.
*/

use std::collections::HashMap;

fn is_anagram(input1: String, input2: String) -> bool {
    /*
    Time complexity
    - First loop to create a HashMap of chars from first input: O(N)
    - Second loop to check the presence of chars in char_map from second input: O(N)
    - Third loop validates the content of map, max iterations 26: O(1)
    - Total time complexity: O(N) + O(N) = 2*O(N) + O(1)
    - Ignoring constants: O(N)

    Space complexity
    - Space required to create a hash set with max key value of 26.
      So constant space required irrespective of input size, therefore O(1)
    - Total space complexity: O(1)
    */
    let mut char_map: HashMap<char, i8> = HashMap::new();

    for char in input1.chars() {
        match char_map.get(&char) {
            Some(count) => {
                char_map.insert(char, count + 1);
            }
            None => {
                char_map.insert(char, 1);
            }
        }
    }

    for char in input2.chars() {
        match char_map.get(&char) {
            Some(count) => {
                char_map.insert(char, count - 1);
            }
            None => return false,
        }
    }

    for value in char_map.values() {
        if *value != 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn should_return_true_for_valid_anagram() {
        let cases = [
            ("listen", "silent"),
            ("aab", "aba"),
            ("race", "care"),
            ("aabb", "bbaa"),
            ("abcde", "edcba"),
        ];

        for (input1, input2) in cases {
            println!(
                "Testing true case: input1='{}', input2='{}'",
                input1, input2
            );
            let input1 = String::from_str(input1).unwrap();
            let input2 = String::from_str(input2).unwrap();
            assert!(is_anagram(input1, input2));
        }
    }

    #[test]
    fn should_return_false_for_invalid_anagram() {
        let cases = [
            // ("rat", "car"),
            // ("hello", "world"),
            ("aab", "ab"),
            ("abc", "aabc"),
            ("aabc", "abcc"),
        ];

        for (input1, input2) in cases {
            println!(
                "Testing false case: input1='{}', input2='{}'",
                input1, input2
            );
            let input1 = String::from_str(input1).unwrap();
            let input2 = String::from_str(input2).unwrap();
            assert!(!is_anagram(input1, input2));
        }
    }
}
