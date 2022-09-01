fn main() {
    println!("{:?}", two_sum(vec![2,7,11,15], 9));
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;  
    let mut index_hashmap = HashMap::with_capacity(nums.len());

    for (idx, &n) in nums.iter().enumerate() {
        let y = target - n;
        if let Some(&i) = index_hashmap.get(&y) {
            return vec![i as i32, idx as i32];
        } else {
            index_hashmap.insert(n, idx);
        }
    }

    vec![]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_1() {
        assert_eq!(vec![0,1], two_sum(vec![2,7,11,15], 9))
    }

    #[test]
    fn test_two_sum_2() {
        assert_eq!(vec![1,2], two_sum(vec![3,2,4], 6))
    }

    #[test]
    fn test_two_sum_3() {
        assert_eq!(vec![0,1], two_sum(vec![3,3], 6))
    }
}