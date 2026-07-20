impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1_i32; nums.len()];
        let mut product = 1;

        for i in 0..nums.len() {
            result[i] = product;
            product *= nums[i]
        }

        product = 1;
        for i in (0..nums.len()).rev() {
            result[i] *= product;
            product *= nums[i]
        }

        result
    }
}