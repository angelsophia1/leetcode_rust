#[allow(dead_code)]
pub struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let mut e1_iter = event1.into_iter();
        let mut e2_iter = event2.into_iter();
        let e1_s = Self::get_minutes(e1_iter.next().unwrap());
        let e1_e = Self::get_minutes(e1_iter.next().unwrap());
        let e2_s = Self::get_minutes(e2_iter.next().unwrap());
        let e2_e = Self::get_minutes(e2_iter.next().unwrap());
        return e1_s <= e2_e && e2_s <= e1_e;
    }
    fn get_minutes(t: String) -> u32 {
        if let Some((h_string, m_string)) = t.split_once(':') {
            let h: u32 = h_string.parse().unwrap();
            let m: u32 = m_string.parse().unwrap();
            return h * 60 + m;
        }
        unreachable!("`:` should be provided in the string: t.");
    }
}