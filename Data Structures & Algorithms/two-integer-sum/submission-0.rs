impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            let diff = target - num;

            if let Some(diff_idx) =  num_map.get(&diff) {
                return vec![*diff_idx, idx as i32];
            }
            else {
                num_map.entry(num).or_insert(idx as i32);
            }
        }

        unreachable!("Invalid input")
    }
}
