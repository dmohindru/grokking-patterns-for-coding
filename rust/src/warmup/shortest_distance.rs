/*
Problem Statement
Given an array of strings words and two different strings that already exist in the array word1 and word2, return the shortest distance between these two words in the list.

Example 1:

Input: words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"], word1 = "fox", word2 = "dog"
Output: 5
Explanation: The distance between "fox" and "dog" is 5 words.
Example 2:

Input: words = ["a", "c", "d", "b", "a"], word1 = "a", word2 = "b"
Output: 1
Explanation: The shortest distance between "a" and "b" is 1 word. Please note that "a" appeared twice.
Example 3:

Input: words = ["a", "b", "c", "d", "e"], word1 = "a", word2 = "e"
Output: 4
Explanation: The distance between "a" and "e" is 4 words.
Constraints:

2 <= words.length <= 3 * 104
1 <= words[i].length <= 10
words[i] consists of lowercase English letters.
word1 and word2 are in words.
word1 != word2
*/
fn shortest_distance(input_array: &[&str], first: &str, second: &str) -> i32 {
    /*
    Time complexity
    - Loop is executed only for number of elements in input_array. Therefore O(N)

    Space complexity
    - Space used in independent of input array size. Therefore O(1)
    */
    let mut first_index: i32 = -1;
    let mut second_index = -1;

    for (index, input) in input_array.iter().enumerate() {
        if *input == first {
            // This is seem first time
            if first_index < 0 {
                first_index = index as i32;
            } else if second_index > -1 {
                // We have seen both the input string by now
                let i = index as i32;
                first_index = swap_shorter_index(&i, &first_index, &second_index);
            }
        }

        if *input == second {
            // This is seem first time
            if second_index < 0 {
                second_index = index as i32;
            } else if first_index > -1 {
                // We have seen both the input string by now
                let i = index as i32;
                second_index = swap_shorter_index(&i, &second_index, &first_index);
            }
        }
    }
    (first_index - second_index).abs()
}

fn swap_shorter_index(curr_index: &i32, comparing: &i32, comparing_to: &i32) -> i32 {
    let old_distance = (comparing - comparing_to).abs();
    let new_distance = (curr_index - comparing_to).abs();
    if new_distance < old_distance {
        *curr_index
    } else {
        *comparing
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_correct_shortest_distance() {
        let words = [
            "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog",
        ];
        let word1 = "fox";
        let word2 = "dog";
        println!(
            "Testing words={:?}, word1={}, word2={}",
            &words, &word1, &word2
        );
        assert_eq!(5, shortest_distance(&words, &word1, &word2));

        let words = ["a", "c", "d", "b", "a"];
        let word1 = "a";
        let word2 = "b";
        println!(
            "Testing words={:?}, word1={}, word2={}",
            &words, &word1, &word2
        );
        assert_eq!(1, shortest_distance(&words, &word1, &word2));

        let words = ["a", "b", "c", "d", "e"];
        let word1 = "a";
        let word2 = "e";
        println!(
            "Testing words={:?}, word1={}, word2={}",
            &words, &word1, &word2
        );
        assert_eq!(4, shortest_distance(&words, &word1, &word2));
    }
}
