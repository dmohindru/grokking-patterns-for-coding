/*
Given a string s, reverse only all the vowels in the string and return it.

The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.

Example 1:

Input: s= "hello"
Output: "holle"
Example 2:

Input: s= "AEIOU"
Output: "UOIEA"
Example 3:

Input: s= "DesignGUrus"
Output: "DusUgnGires"
Constraints:

1 <= s.length <= 3 * 105
s consist of printable ASCII characters.
*/

fn is_palindrome(input: String) -> bool {
    /*
    Time Complexity
    - sanitizing input: O(N)
    - converting sanitized input to byte array: O(N)
    - palindrome loop max N/2 times: O(N/2)
    - Total: O(N) + O(N) + O(N/2) = 2 O(N) + O(N/2)
    - ignoring constants: O(N)

    Space Complexity
    - sanitizing input: O(N)
    - Byte array: O(N)
    - Total: O(N) + O(N) = 2*O(N)
    - Ignoring constants = O(N)
     */
    let sanitized_input: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let byte_string = sanitized_input.as_bytes();
    let mut left: usize = 0;
    let mut right: usize = byte_string.len() - 1;
    while left < right {
        if byte_string[left] != byte_string[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn should_return_true_for_valid_palindrome() {
        let input1 = String::from_str("A man, a plan, a canal, Panama!").unwrap();
        assert!(is_palindrome(input1));

        let input2 = String::from_str("Was it a car or a cat I saw?").unwrap();
        assert!(is_palindrome(input2));
    }

    #[test]
    fn should_return_false_for_not_valid_palindrome() {
        let input1 = String::from_str("This is not a palindrome").unwrap();
        assert!(!is_palindrome(input1));
    }
}
