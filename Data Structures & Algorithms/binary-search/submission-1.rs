impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0i32;
        let mut r = nums.len() as i32 - 1;

        while l <= r {
            let mid  = r - ( r - l ) / 2;
            match nums[mid as usize].cmp(&target) {
                std::cmp::Ordering::Equal => return mid,
            std::cmp::Ordering::Greater => r = mid - 1,
            std::cmp::Ordering::Less => l = mid + 1,
            }
        }
        -1
    }
}
