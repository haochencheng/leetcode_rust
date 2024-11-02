pub trait Solution {
    fn inventory_management(stock: Vec<i32>, cnt: i32) -> Vec<i32>;

    fn quick_sort(arr: &mut [i32]);

    fn partition(arr: &mut [i32]) -> usize;
}

pub struct InventoryManagement {}

// LCR 169. 招式拆解 II
impl Solution for InventoryManagement {
    fn inventory_management(arr: Vec<i32>, cnt: i32) -> Vec<i32> {
        let mut arr: Vec<i32> = arr.clone();
        Self::quick_sort(&mut arr);
        println!("after sort arr:{:#?}", arr);
        arr[0..cnt as usize].to_vec()
    }

    fn quick_sort(arr: &mut [i32]) {
        if arr.len() <= 1 {
            return;
        }
        let pivot_index = Self::partition(arr);
        Self::quick_sort(&mut arr[0..pivot_index]);
        Self::quick_sort(&mut arr[pivot_index + 1..]);
    }

    fn partition(arr: &mut [i32]) -> usize {
        let pivot = arr[arr.len() - 1];
        let mut i = 0;

        for j in 0..arr.len() - 1 {
            if arr[j] > pivot {
                arr.swap(i, j);
                // println!("after sort arr:{:#?}", arr);
                i += 1;
            }
        }
        arr.swap(i, arr.len() - 1);
        // println!("After placing pivot arr: {:?}", arr);
        i
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn judge_circle() {
        let sales = vec![2, 5, 7, 4];
        let result = vec![2];
        assert_eq!(InventoryManagement::inventory_management(sales, 1), result);
        // let sales = vec![0, 2, 3, 6];
        // let result = vec![0, 2];
        // assert_eq!(InventoryManagement::inventory_management(sales, 2), result);
    }
}
