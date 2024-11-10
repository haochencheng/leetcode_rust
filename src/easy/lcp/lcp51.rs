use std::i32;

struct Solution {}

impl Solution {
    pub fn perfect_menu(
        mut materials: Vec<i32>,
        cookbooks: Vec<Vec<i32>>,
        attribute: Vec<Vec<i32>>,
        limit: i32,
    ) -> i32 {
        let mut ans = -1;
        if materials.is_empty() || cookbooks.is_empty() || attribute.is_empty() {
            return ans;
        }
        let mut exists = vec![false; cookbooks.len()];

        ans = Self::dfs(
            &mut materials,
            &cookbooks,
            &attribute,
            limit,
            &mut exists,
            0,
            0,
            ans,
        );
        ans
    }

    fn dfs(
        materials: &mut Vec<i32>,
        cookbooks: &Vec<Vec<i32>>,
        attribute: &Vec<Vec<i32>>,
        limit: i32,
        exists: &mut Vec<bool>,
        sumx: i32,
        sumy: i32,
        mut ans: i32,
    ) -> i32 {
        if sumy >= limit {
            ans = ans.max(sumx);
        }

        for i in 0..cookbooks.len() {
            if exists[i] {
                continue;
            }
            let need = &cookbooks[i];
            let mut can = true;
            for j in 0..need.len() {
                if need[j] > materials[j] {
                    can = false;
                    break;
                }
            }
            if can {
                exists[i] = true;
                for j in 0..need.len() {
                    materials[j] -= need[j];
                }
                let ans_tmp = Self::dfs(
                    materials,
                    cookbooks,
                    attribute,
                    limit,
                    exists,
                    sumx + attribute[i][0],
                    sumy + attribute[i][1],
                    ans,
                );
                ans = ans.max(ans_tmp);
                for j in 0..need.len() {
                    materials[j] += need[j];
                }
                exists[i] = false;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn min_num_booths() {
        let materials = vec![3, 2, 4, 1, 2];
        let cookbooks = vec![
            vec![1, 1, 0, 1, 2],
            vec![2, 1, 4, 0, 0],
            vec![3, 2, 4, 1, 0],
        ];
        let attribute = vec![vec![3, 2], vec![2, 4], vec![7, 6]];
        let limit = 5;
        assert_eq!(
            Solution::perfect_menu(materials, cookbooks, attribute, limit),
            7
        );
        let materials = vec![12, 13, 12, 20, 6];
        let cookbooks = vec![
            vec![17, 14, 15, 7, 20],
            vec![14, 12, 9, 2, 7],
            vec![18, 8, 5, 9, 2],
            vec![4, 6, 14, 9, 6],
            vec![10, 0, 20, 8, 12],
            vec![12, 1, 19, 5, 13],
            vec![4, 9, 10, 15, 11],
            vec![6, 17, 6, 15, 10],
        ];
        let attribute = vec![
            vec![4, 4],
            vec![8, 20],
            vec![9, 5],
            vec![0, 4],
            vec![0, 18],
            vec![6, 4],
            vec![14, 1],
            vec![7, 13],
        ];
        let limit = 64;
        assert_eq!(
            Solution::perfect_menu(materials, cookbooks, attribute, limit),
            -1
        );
    }
}
