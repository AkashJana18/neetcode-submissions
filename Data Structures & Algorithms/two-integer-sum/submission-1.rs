impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hm = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = hm.get(&complement) {
                return vec![j as i32, i as i32];
            }
            hm.insert(num, i);
        }
        vec![]
    }
}
