impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut hs = HashSet::new();
        for num in nums {
            if hs.contains(&num) {
                return true;
            }
            hs.insert(num);
        }
        false
    }
}
