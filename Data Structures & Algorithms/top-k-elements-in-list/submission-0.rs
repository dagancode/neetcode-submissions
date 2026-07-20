impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        let mut buckets = vec![vec![]; (nums.len() + 1) as usize];

        nums.into_iter().for_each(|n| {
            map.entry(n).and_modify(|v| *v += 1).or_insert(1);
        });

        map.iter().for_each(|(key, val)| {
            buckets[*val as usize].push(*key);
        });

        buckets.reverse();
        buckets.into_iter().flatten().take(k as usize).collect()
    }
}