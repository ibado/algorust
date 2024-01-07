struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) {
        for (i, el) in nums2.iter().enumerate() {
            let idx = m as usize + i;
            nums1[idx] = *el;
        }
        nums1.sort();
    }
}

pub fn run(execute: bool) {
    if !execute {
        return;
    }

    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];

    println!("nums1: {:?}", &nums1);
    Solution::merge(&mut nums1, 3, &mut nums2, 3);
    println!("nums1: {:?}, nums2: {:?}", &nums1, &nums2);
}
