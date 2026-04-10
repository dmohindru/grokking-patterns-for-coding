/*
Problem Statement
A pangram is a sentence where every letter of the English alphabet appears at least once.

Given a string sentence containing English letters (lower or upper-case), return true if sentence is a pangram, or false otherwise.

Note: The given sentence might contain other characters like digits or spaces, your solution should handle these too.

Example 1:

Input: sentence = "TheQuickBrownFoxJumpsOverTheLazyDog"
Output: true
Explanation: The sentence contains at least one occurrence of every letter of the English alphabet either in lower or upper case.
Example 2:

Input: sentence = "This is not a pangram"
Output: false
Explanation: The sentence doesn't contain at least one occurrence of every letter of the English alphabet.
Constraints:

1 <= sentence.length <= 1000
sentence consists of lower or upper-case English letters.

*/

fn pengram(input: String) -> bool {
    let char = 'A';

    for c in input.chars() {}
    todo!()
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn should_return_false_if_sentence_is_not_pengram() {
        let not_pengram_str = String::from_str("This is not a pangram").unwrap();
        let is_pengram = pengram(not_pengram_str);
        assert!(!is_pengram);
    }

    #[test]
    fn should_return_true_if_sentence_is_pengram() {
        let pengram_str = String::from_str("TheQuickBrownFoxJumpsOverTheLazyDog").unwrap();
        let is_pengram = pengram(pengram_str);
        assert!(is_pengram);
    }
}
