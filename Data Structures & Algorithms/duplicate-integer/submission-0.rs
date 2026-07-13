impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            for j in (i+1)..nums.len() {
                if nums.get(i) == nums.get(j) {
                    return true;
                }
            }
        }
        false
    }
}
