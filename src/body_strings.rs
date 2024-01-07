struct Solution;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let mut indexchars = std::collections::HashMap::new();

        for (i, c) in s.chars().enumerate() {
            indexchars
                .entry((i, c))
                .and_modify(|(count, _)| *count += 1)
                .or_insert((1, "subject"));
        }

        for (i, c) in goal.chars().enumerate() {
            indexchars
                .entry((i, c))
                .and_modify(|(count, _)| *count += 1)
                .or_insert((1, "goal"));
        }

        let diff = indexchars.values().filter(|(c, _)| *c != 2).count();

        diff == 4 && Solution::swappable(&indexchars)
            || diff == 0 && Solution::has_repeated(&indexchars)
    }

    fn has_repeated(
        entries: &std::collections::HashMap<(usize, char), (usize, &'static str)>,
    ) -> bool {
        let mut m = std::collections::HashMap::new();
        for e in entries.into_iter() {
            if let Some(_) = m.get_mut(&e.0 .1) {
                return true;
            } else {
                m.insert(e.0 .1, 1);
            }
        }

        false
    }

    fn swappable(
        entries: &std::collections::HashMap<(usize, char), (usize, &'static str)>,
    ) -> bool {
        let subject = entries
            .iter()
            .filter(|(_, (c, n))| *c == 1 && *n == "subject")
            .collect::<Vec<_>>();

        let goal = entries
            .iter()
            .filter(|(_, (c, n))| *c == 1 && *n == "goal")
            .collect::<Vec<_>>();

        if subject.len() != 2 || goal.len() != 2 {
            return false;
        }

        for s in subject.iter() {
            if let None = goal.iter().find(|e| e.0 .0 != s.0 .0 && e.0 .1 == s.0 .1) {
                return false;
            }
        }

        true
    }
}

pub fn run(execute: bool) {
    if !execute {
        return;
    }
    let r = Solution::buddy_strings("abcaa".to_string(), "abcbb".to_string());
    println!("result: {r}");
}
