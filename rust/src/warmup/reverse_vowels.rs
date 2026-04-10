// Given a string s, reverse only all the vowels in the string and return it.

// The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.

// Example 1:

// Input: s= "hello"
// Output: "holle"
// Example 2:

// Input: s= "AEIOU"
// Output: "UOIEA"
// Example 3:

// Input: s= "DesignGUrus"
// Output: "DusUgnGires"
// eiUu
// DusUgnGires
fn reverse_vowels(input: String) -> String {
    /*
    Time Complexity: log(n)
    - Converting String to mutable Vec n = log(n)
    - While loop will run max for n - 1 in worst case scenario e.g "ABCD" = log(n)
    - Converting mutable vec to string back = log(n)
    - Total = log(n) + log(n) + log(n) = 3log(n). Ignoring constant term 3, final time complexity log(n)

    Space Complexity: log(N)
    - Additional space required for creating new string of n chars. Hence space complexity log(n).
     */
    let mut chars: Vec<char> = input.chars().collect();
    let mut left: usize = 0;
    let mut right = chars.len() - 1;
    while left < right {
        if is_vowel(chars.get(left).unwrap()) && is_vowel(chars.get(right).unwrap()) {
            chars.swap(left, right);
            left += 1;
            right -= 1;
        } else if !is_vowel(chars.get(left).unwrap()) {
            left += 1;
        } else if !is_vowel(chars.get(right).unwrap()) {
            right -= 1;
        }
    }

    chars.into_iter().collect()
}

fn is_vowel(c: &char) -> bool {
    let c = c.to_ascii_lowercase();
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn should_print_reverse_vowels() {
        let input1 = String::from_str("hello").unwrap();
        assert_eq!(String::from_str("holle").unwrap(), reverse_vowels(input1));

        let input2 = String::from_str("AEIOU").unwrap();
        assert_eq!(String::from_str("UOIEA").unwrap(), reverse_vowels(input2));

        let input3 = String::from_str("DesignGUrus").unwrap();
        assert_eq!(
            String::from_str("DusUgnGires").unwrap(),
            reverse_vowels(input3)
        );
    }
}
