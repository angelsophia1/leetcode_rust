use std::collections::HashMap;
#[allow(dead_code)]
pub struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_map = HashMap::<i32, i32>::new();
        for (idx, num) in nums.into_iter().enumerate() {
            let diff = target - num;
            match nums_map.get(&diff) {
                Some(&diff_idx) => {
                    return vec![diff_idx, idx as i32];
                }
                None => {
                    nums_map.insert(num, idx as i32);
                }
            }
        }
        vec![]
    }
}