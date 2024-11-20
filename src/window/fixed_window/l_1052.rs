struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let k = minutes as usize;
        if k >= customers.len() {
            return customers.iter().sum();
        }
        let mut sum: i32 =
            customers.iter().enumerate().fold(
                0,
                |acc, (i, &x)| {
                    if grumpy[i] == 0 {
                        acc + x
                    } else {
                        acc
                    }
                },
            );
        let n = grumpy.len();
        // Initial sum for the first window
        let mut ans = sum;
        // println!("sum====={}", sum);
        for i in 0..n {
            // Slide the window to the right by removing the leftmost element and adding the next element
            if i >= k {
                if grumpy[i - k] == 1 {
                    sum -= customers[i - k];
                }
            }
            if grumpy[i] == 1 {
                sum += customers[i];
            }
            ans = ans.max(sum);
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_unique_split() {
        let customers = vec![8, 8];
        let grumpy = vec![1, 0];
        let minutes = 2;
        let ans = 16;
        assert_eq!(Solution::max_satisfied(customers, grumpy, minutes), ans);

        let customers = vec![1, 0, 1, 2, 1, 1, 7, 5];
        let grumpy = vec![0, 1, 0, 1, 0, 1, 0, 1];
        let minutes = 3;
        let ans = 16;
        assert_eq!(Solution::max_satisfied(customers, grumpy, minutes), ans);

        let customers = vec![4, 10, 10];
        let grumpy = vec![1, 1, 0];
        let minutes = 2;
        let ans = 24;
        assert_eq!(Solution::max_satisfied(customers, grumpy, minutes), ans);
    }
}
