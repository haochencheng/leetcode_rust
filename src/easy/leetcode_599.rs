// 599. 两个列表的最小索引总和
// 思路：hahsmap记录list1的值和索引，记录最小索引和，如果有新的跟新最小索引和数据

use std::collections::HashMap;

pub trait Solution {
    fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> ;
}


pub struct FindRestaurant {}

impl Solution for FindRestaurant {

    fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String>   {
        if list1.is_empty()||list2.is_empty() {
            return Vec::new();
        }
        let mut list1_map:HashMap<String,usize> =HashMap::new();
        for _i in 0..list1.len() {
            list1_map.insert(list1[_i].to_string(), _i);
        }
        let mut result = Vec::new();
        let mut max=usize::max_value();
        for i in 0..list2.len() {
            if let Some(x) = list1_map.get(&list2[i]) {
                if (x+i)<max {
                    max=x+i;
                    result.clear();
                    result.push(list2[i].to_string());
                }else if (x+i) == max{
                    result.push(list2[i].to_string());
                }
            };
        
        
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn max_unique_split() {
        let list1: Vec<String> = vec!["Shogun", "Tapioca Express", "Burger King", "KFC"].iter().map(|&s| s.into()).collect();
        let list2: Vec<String> = vec!["Piatti", "The Grill at Torrey Pines", "Hungry Hunter Steakhouse", "Shogun"].iter().map(|&s| s.into()).collect();
        let result: Vec<String>  =vec!["Shogun"].iter().map(|&s| s.into()).collect();
        assert_eq!(FindRestaurant::find_restaurant(list1,list2),result);
        let list1: Vec<String> = vec!["Shogun","Tapioca Express","Burger King","KFC"].iter().map(|&s| s.into()).collect();
        let list2: Vec<String> = vec!["KFC","Shogun","Burger King"].iter().map(|&s| s.into()).collect();
        let result: Vec<String>  =vec!["Shogun"].iter().map(|&s| s.into()).collect();
        assert_eq!(FindRestaurant::find_restaurant(list1,list2),result);
    }
}
