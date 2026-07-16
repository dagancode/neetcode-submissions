impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut char_count = [0_i32; 26];
        for c in s.chars() {
            let idx = (c as u8 - b'a') as usize;
            char_count[idx] += 1;
        }

        for c in t.chars() {
            let idx = (c as u8 - b'a') as usize;
            if char_count[idx] >= 1 {
                char_count[idx] -= 1;
            } else {
                return false;
            }
        }
        
        true
    }
}