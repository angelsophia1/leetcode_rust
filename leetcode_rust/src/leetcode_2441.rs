use std::collections::HashSet;
#[allow(dead_code)]
pub struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut nums_set = HashSet::new();
        let mut max_k = -1;
        for num in nums {
            if nums_set.contains(&-num) {
                max_k = max_k.max(num.abs());
            }
            nums_set.insert(num);
        }
        max_k
    }
}