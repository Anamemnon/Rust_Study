// not ready

fn main() {
    println!("{:?}", Solution::max_subsequence(vec![-1,-2,3,4], 3));
}

struct Solution;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() <= k as usize {
            return nums
        }

        let mut min = nums[0];
        let mut min_index = 0;
        let start: usize = 0;
        let end: usize = k as usize;

        nums.iter()
            .take(k as usize)
            .enumerate()
            .for_each(|(i, v)| {
                if v < &min {
                    min = *v;
                    min_index = i;
                };
            }
        );

        let mut answer = [&nums[start..min_index], &nums[(min_index+1)..=end]].concat();

        let mut sum = nums[start..end].iter().sum::<i32>();

        for i in k+1..nums.len() as i32 {
            let local_sum = sum - min.abs() + nums[i as usize];
            println!("{} {}", sum, local_sum);

            if sum < local_sum {
                sum = local_sum;
            }
        }

        answer
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::max_subsequence(vec![-1,-2,3,4], 3);
        assert_eq!(result, vec![-1,3,4]);
    }
}
