impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let trim: String = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
        let rev: String = trim.chars().rev().collect();
        trim == rev 
    }
}
