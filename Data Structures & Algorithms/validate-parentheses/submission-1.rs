impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let pairs = std::collections::HashMap::from([('[', ']'), ('(', ')'), ('{', '}')]);

        for bracket in s.chars() {
            if pairs.contains_key(&bracket) {
                stack.push(bracket);
            } else {
                if let Some(opening_bracket) = stack.last() {
                    if let Some(matched_closing) = pairs.get(&opening_bracket) {
                        if bracket == *matched_closing {
                            stack.pop();
                        }
                        else {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
            }
        }

        stack.len() == 0
    }
}
