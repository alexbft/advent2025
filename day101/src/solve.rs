use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let target = parse_target(parts[0]);
        let buttons = parse_buttons(&parts[1..parts.len() - 1]);
        let mut visited = HashSet::new();
        visited.insert(0);
        let mut step = 0;
        let mut reach = vec![0u32];
        result += 'bfs: loop {
            if reach.is_empty() {
                panic!("Unreachable");
            }
            step += 1;
            let mut next_reach = Vec::new();
            for cur in &reach {
                for b in &buttons {
                    let next = cur ^ b;
                    if next == target {
                        break 'bfs step;
                    }
                    if !visited.contains(&next) {
                        visited.insert(next);
                        next_reach.push(next);
                    }
                }
            }
            reach = next_reach;
        }
    }
    result
}

fn parse_target(target: &str) -> u32 {
    let mut result = 0;
    for c in target[1..target.len() - 1].chars().rev() {
        result *= 2;
        if c == '#' {
            result += 1;
        }
    }
    result
}

fn parse_buttons(buttons_str: &[&str]) -> Vec<u32> {
    buttons_str
        .iter()
        .map(|button_str| {
            let mut result = 0;
            for place_str in button_str[1..button_str.len() - 1].split(',') {
                let place: u32 = place_str.parse().unwrap();
                result += 1 << place;
            }
            result
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "};
        assert_eq!(solve(input), 7);
    }
}
