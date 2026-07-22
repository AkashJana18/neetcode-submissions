

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq = HashMap::new();

        for num in &nums {
            *freq.entry(*num).or_insert(0) += 1;
        }

        // bucket[i] = numbers occurring i times
        let mut buckets = vec![Vec::new(); nums.len() + 1];

        for (num, count) in freq {
            buckets[count as usize].push(num);
        }

        let mut ans = Vec::new();

        for i in (1..buckets.len()).rev() {
            for &num in &buckets[i] {
                ans.push(num);

                if ans.len() == k as usize {
                    return ans;
                }
            }
        }

        ans
    }
}
