impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut encoded_string = String::new();

        strs.into_iter().for_each(|s| {
            let len = s.len();
            encoded_string += &format!("{}#{}", len, s);
        });

        encoded_string
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut idx = 0;
        let mut decoded_strings = Vec::new();

        while let Some(hash_idx) = s[idx..].find("#") {
            let abs_hash_idx = idx + hash_idx;
            let len = s[idx..abs_hash_idx].parse::<u32>().expect("malformed encoded string: length prefix is not a valid number");

            let str = &s[(abs_hash_idx + 1)..(abs_hash_idx + 1 + len as usize)];
            decoded_strings.push(str.to_string());
            idx = abs_hash_idx + 1 + len as usize;
        }

        decoded_strings
    }
}