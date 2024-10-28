// LCR 190. 加密运算
/**
 * Let's learn by example. Imagine that a = 3 and b = 5

In binary notation they are a = 0011 and b = 0101

XOR: a^b is XOR operator. When compare two bits it returns 0 if they are same and 1 if they are different. 01^10 => 11

So when we're doing a^b result will be 0110.

AND + SHIFT

a&b performs logical AND operation. It returns 1 only when a = b = 1.

In our case the result is 0001

<< shifts it(adds 0 on the right side) and result became 0010 which sets carry variable true. (only 0000 will be false).

Next iterations:

Everything repeats but now a = 0110 and b = 0010 (Sum and carry from last execution)

Now a^b = 0100 and (a&b)<<1 = 0100

Repeating again.

Now a^b = 0000 and (a&b)<<1 = 1000

And again.

Now a^b = 1000 and (a&b)<<1 = 0000. Now carry is finally false. And we're returning 1000 which is decimal 8.

Everything worked fine since 3+5=8
 */

pub trait Solution {
    fn encryption_calculate(data_a: i32, data_b: i32) -> i32;

}


pub struct EncryptionCalculate {}

impl Solution for EncryptionCalculate {

    fn  encryption_calculate(data_a: i32, data_b: i32) -> i32 {
        let mut a= data_a;
        let mut b= data_b;
        while b!=0 {
            // 进位
            let carry = (a&b)<<1;
            // 求和 
            a=a^b;
            // 进位作为输入
            b=carry;
        }
        a
    }

    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        let (data_a,data_b,result) = (5,-1,4);
        assert_eq!(EncryptionCalculate::encryption_calculate(data_a,data_b),result);
        
        
    }
}
