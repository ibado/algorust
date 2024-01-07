struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let xstr = x.to_string();
        let len = xstr.len();
        let mut stack = std::collections::VecDeque::new();
        let odd_len = len % 2 != 0;
        for (i, c) in xstr.chars().enumerate() {
            match i {
                idx if idx < len / 2 => stack.push_back(c),
                idx if idx == len / 2 && odd_len => continue,
                _ => match stack.pop_back() {
                    Some(e) if e == c => continue,
                    _ => return false,
                },
            }
        }

        stack.is_empty()
    }
}

pub fn run(execute: bool) {
    if !execute {
        return;
    }
    let r = Solution::is_palindrome(10);
    println!("is palindrome: {r}");
}
