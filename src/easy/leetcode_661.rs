// 661. 图片平滑器

pub trait Solution {
    fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

pub struct ImageSmoother {}

impl Solution for ImageSmoother {
    fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if img.is_empty() {
            return Vec::new();
        }

        let (x, y) = (img.len(), img[0].len());
        // println!("x,y===={},{}",x,y);
        let mut result = vec![vec![0; y]; x];
        let mut _i = 0;
        for i in 0..x {
            for j in 0..y {
                let mut x_start = 0;
                if i >= 1 {
                    x_start = (i - 1) as usize;
                }
                let mut x_end: usize = i + 2;
                if i + 2 > x {
                    x_end = x as usize;
                }
                let mut y_start: usize = 0;
                if j > 1 {
                    y_start = j - 1 as usize;
                }
                let mut y_end: usize = j + 2;
                if j + 2 > y {
                    y_end = y as usize;
                }
                let mut sum = 0;
                let mut count = 0;
                // println!("x_start..x_end ======,y_start..y_end ======{},{},{},{}",x_start,x_end,y_start,y_end);
                for k in x_start..x_end {
                    for l in y_start..y_end {
                        sum = sum + img[k][l];
                        count += 1;
                        _i += 1;
                    }
                }
                // println!("sum======,count======,{},{}",sum,count);
                let avg = (sum / count) as f64;
                // let avg= img[i-1][j-1]+img[i-1][j]+img[i-1][j+1]+
                // img[i][j]+img[i][j+1]+img[i][j-1]
                // +img[i+1][j-1]+img[i+1][j+1]+img[i+1][j];
                result[i][j] = avg.floor() as i32;
            }
        }
        // println!("result-=-======={:#?}",result);
        println!("_i======{}", _i);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_circle() {
        let img: Vec<Vec<i32>> = vec![[1, 1, 1].to_vec(), [1, 0, 1].to_vec(), [1, 1, 1].to_vec()];
        let result: Vec<Vec<i32>> =
            vec![[0, 0, 0].to_vec(), [0, 0, 0].to_vec(), [0, 0, 0].to_vec()];
        assert_eq!(ImageSmoother::image_smoother(img), result);
        let img: Vec<Vec<i32>> = vec![
            [100, 200, 100].to_vec(),
            [200, 50, 200].to_vec(),
            [100, 200, 100].to_vec(),
        ];
        let result: Vec<Vec<i32>> = vec![
            [137, 141, 137].to_vec(),
            [141, 138, 141].to_vec(),
            [137, 141, 137].to_vec(),
        ];
        assert_eq!(ImageSmoother::image_smoother(img), result);
    }
}
