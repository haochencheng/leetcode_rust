// 228. 汇总区间
// [0,1,2,4,5] 就不可以是0-3因为 3 不在 [0,1,2,4,5] 
// 如果数字连续就在一个区间，不连续从新开始一个区间，如果只有一个数字输出这个单独的数字
// 开始指针start ,和下一个数字判断，如果下一个数字等于当前数字+1，继续寻找结束数字
// 当前数字+1不等于下一个数字，区间结束写入结果。从新开始一个区间


use core::num;

pub trait Solution {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String>;
}


pub struct SummaryRanges {}

impl Solution for SummaryRanges {
    fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len()==0 {
            return Vec::new();
        }
        let mut result:Vec<String> = Vec::new();
        let mut start=nums[0];
        let mut is_start=true;
        for i in 0..nums.len()  {
            // println!("i===={}",i);
            if is_start {
                start=nums[i];
                is_start=false;
            }
            let k=i+1;
            if i!=nums.len()-1 && nums[k]==nums[i]+1 {
                continue;
            }else {
                if start==nums[i] {
                    result.push(format!("{}", start));
                }else {
                    result.push(format!("{}->{}", start,nums[i] ));
                }
                is_start=true;
            }
        }  
        // println!("result => {:#?}",result);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let nums = vec![0,1,2,4,5,7];
        let result =vec!["0->2","4->5","7"];
        assert_eq!(<SummaryRanges as Solution>::summary_ranges(nums),result);
        let nums = vec![-1,0,2,9];
        let result =vec!["-1->0","2","9"];
        assert_eq!(<SummaryRanges as Solution>::summary_ranges(nums),result);
        
    }
}
