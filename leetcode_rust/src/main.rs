mod two_sum;

fn main() {
    println!("Hello World");
}

#[cfg(test)]
mod test {
    use crate::two_sum::Solution;

    #[test]
    fn test_two_sum() {
        let mut two_sum_res = Solution::two_sum(vec![2,7,11,15], 9);
        two_sum_res.sort();
        assert_eq!(vec![0, 1], two_sum_res);
    }
}