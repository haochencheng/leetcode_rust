// LCR 181. 字符串中的单词反转

pub trait Solution {
    fn file_combination(target: i32) -> Vec<Vec<i32>>  ;

}


pub struct FileCombination {}

impl Solution for FileCombination {

    fn file_combination(target: i32) -> Vec<Vec<i32>>  {
        if target <= 0 {
            return Vec::new();
        }
        let mut l = 1;
        let mut r=2;
        let mut sum=3;
        let mut res=Vec::new();
        while l< (target+1) /2 {
            if sum<target {
                r+=1;
                sum += r;
            }else if sum > target {
                sum-=l;
                l+=1;
            }else {
                let mut tmp:Vec<i32> =  Vec::new();
                for i in l..r+1{
                    tmp.push(i);
                }
                res.push(tmp);
                sum -= l;
                l += 1;
            }
        }
        res
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        let targrt = 12;
        let result = [[3, 4, 5]];
        assert_eq!(FileCombination::file_combination(targrt),result);
        let targrt = 10;
        let result = [[1,2,3,4]];
        assert_eq!(FileCombination::file_combination(targrt),result);
        let targrt = 93;
        let mut result = Vec::new();
        result.push([13,14,15,16,17,18]);
        result.push([30,31,32,0,0,0]);
        result.push([46,47,0,0,0,0]);
        assert_eq!(FileCombination::file_combination(targrt),result);
    }
}
