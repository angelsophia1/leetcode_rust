mod two_sum;
mod leetcode_2441;
mod leetcode_2446;

fn main() {
    println!("Hello World");
}

#[cfg(test)]
mod test {
    use crate::two_sum;
    use crate::leetcode_2441;
    use crate::leetcode_2446;

    #[test]
    fn test_two_sum() {
        let mut two_sum_res = two_sum::Solution::two_sum(vec![2,7,11,15], 9);
        two_sum_res.sort();
        assert_eq!(vec![0, 1], two_sum_res);
    }

    #[test]
    fn test_leetcode_2441() {
        assert_eq!(3, leetcode_2441::Solution::find_max_k(vec![-1, 2, -3, 3]));
        assert_eq!(7, leetcode_2441::Solution::find_max_k(vec![-1,10,6,7,-7,1]));
        assert_eq!(-1, leetcode_2441::Solution::find_max_k(vec![-10,8,6,7,-2,-3]));
    }

    #[test]
    fn test_leetcode_2446() {
        assert_eq!(true, leetcode_2446::Solution::have_conflict(vec!["01:00","02:00"], vec!["01:20","03:00"]));
        assert_eq!(false, leetcode_2446::Solution::have_conflict(vec!["10:00","11:00"], vec!["14:00","15:00"]));
    }
}