use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        println!("{}", line);
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let buttons = parse_buttons(&parts[1..parts.len() - 1]);
        let target = parse_nums(parts[parts.len() - 1]);
        result += Solver::new(buttons, target).solve();
    }
    result
}

struct Solver {
    target: Vec<u32>,
    buttons_by_place: HashMap<u32, Vec<usize>>,
}

impl Solver {
    fn new(buttons: Vec<Vec<u32>>, target: Vec<u32>) -> Solver {
        let mut buttons_by_place = HashMap::new();
        for (i, b) in buttons.into_iter().enumerate() {
            for place in b {
                buttons_by_place
                    .entry(place)
                    .or_insert_with(Vec::new)
                    .push(i);
            }
        }
        // println!("{:?}", buttons_by_place);
        Solver {
            buttons_by_place,
            target,
        }
    }

    fn solve(&self) -> usize {
        self.solve_rec(HashMap::new()).unwrap()
    }

    fn solve_rec(&self, mut fixed: HashMap<usize, u32>) -> Option<usize> {
        //println!("checking: {:?}", fixed);
        let floating = self.buttons_by_place.iter().map(|(&k, v)| {
            (
                k,
                v.iter()
                    .filter(|&button| !fixed.contains_key(button))
                    .collect::<Vec<_>>(),
            )
        });
        let min_floating = floating
            .filter(|(_, v)| v.len() > 0)
            .min_by_key(|(_, v)| v.len());
        match min_floating {
            None => Some(fixed.values().sum::<u32>() as usize),
            Some((place, floating_buttons)) => {
                let target = self.target[place as usize];
                let fixed_sum = self.buttons_by_place[&place]
                    .iter()
                    .map(|btn| {
                        if fixed.contains_key(btn) {
                            fixed[btn]
                        } else {
                            0
                        }
                    })
                    .sum::<u32>();
                let remaining = target - fixed_sum;
                let range = if floating_buttons.len() == 1 {
                    remaining..=remaining
                } else {
                    0..=remaining
                };
                let button = floating_buttons[0];
                let mut result: Option<usize> = None;
                for fixed_value in range.rev() {
                    fixed.insert(*button, fixed_value);
                    if self.check_ok(&fixed) {
                        if let Some(maybe_result) = self.solve_rec(fixed.clone()) {
                            if result.is_none() || maybe_result < result.unwrap() {
                                result = Some(maybe_result);
                            } else {
                                break;
                            }
                        }
                    }
                }
                result
            }
        }
    }

    fn check_ok(&self, fixed: &HashMap<usize, u32>) -> bool {
        for (place, buttons) in &self.buttons_by_place {
            let mut floating = 0;
            let mut fixed_sum = 0;
            for button in buttons {
                if fixed.contains_key(button) {
                    fixed_sum += fixed[button];
                } else {
                    floating += 1;
                }
            }
            let target = self.target[*place as usize];
            if fixed_sum > target {
                return false;
            }
            if floating == 0 && fixed_sum != target {
                return false;
            }
        }
        true
    }
}

fn parse_nums(nums_str: &str) -> Vec<u32> {
    nums_str[1..nums_str.len() - 1]
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_buttons(buttons_str: &[&str]) -> Vec<Vec<u32>> {
    buttons_str.iter().map(|&s| parse_nums(s)).collect()
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
        assert_eq!(solve(input), 33);
    }
}
