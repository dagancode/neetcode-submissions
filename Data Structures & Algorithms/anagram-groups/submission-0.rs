impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map: HashMap<[i32; 26], Vec<String>> = HashMap::new();

        strs.into_iter().for_each(|s| {
            let mut counts = [0_i32; 26];

            s.chars().for_each(|c| {
                let idx = (c as u8 - b'a') as usize;
                counts[idx] += 1;
            });
            anagram_map.entry(counts).and_modify(|v| v.push(s.clone())).or_insert(vec![s]);

        });
        anagram_map.into_values().collect()
    }
}