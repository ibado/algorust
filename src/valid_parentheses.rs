struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let mut stack = std::collections::VecDeque::new();
        for i in s.chars() {
            match i {
                '{' | '[' | '(' => stack.push_back(i),
                '}' | ']' | ')' => match stack.pop_back() {
                    Some(p) if p != Solution::resolve_op(i) => return false,
                    None => return false,
                    _ => (),
                },
                _ => return false,
            }
        }

        stack.is_empty()
    }

    pub fn resolve_op(parentheses: char) -> char {
        match parentheses {
            '}' => '{',
            ']' => '[',
            ')' => '(',
            _ => panic!("invalid parentheses"),
        }
    }
}

pub fn run(execute: bool) {
    if !execute {
        return;
    }
    let input = "))";
    let r = Solution::is_valid(input.to_string());
    println!("input: {input}");
    println!("valid: {r}");
}
