// LCR 173. 点名

pub trait Solution {
    fn take_attendance(records: Vec<i32>) -> i32;
}

pub struct TakeAttendance {}

impl Solution for TakeAttendance {
    fn take_attendance(records: Vec<i32>) -> i32 {
        if records.is_empty() {
            return 0;
        }
        let mut pre = 0;
        for i in 0..records.len() {
            if records[i] != pre {
                return pre;
            } else {
                pre += 1;
            }
        }
        return pre;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let records = vec![0, 1, 2, 3, 5];
        // println!("root ====={:#?}",root);
        assert_eq!(TakeAttendance::take_attendance(records), 4);
        let records = vec![1];
        // println!("root ====={:#?}",root);
        assert_eq!(TakeAttendance::take_attendance(records), 0);
    }
}
