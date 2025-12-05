use std::ops::RangeInclusive;

type RangeVec = Vec<RangeInclusive<u64>>;

pub fn solve(input: &str) -> usize {
    let (fresh_part, values_part) = input.split_once("\n\n").unwrap();
    let fresh = parse_fresh(fresh_part);
    let mut result = 0;
    for value_s in values_part.lines() {
        let value = value_s.parse::<u64>().unwrap();
        if fresh.iter().any(|range| range.contains(&value)) {
            result += 1;
        }
    }
    result
}

fn parse_fresh(str: &str) -> RangeVec {
    str.lines()
        .map(|line| {
            let (l, r) = line.split_once("-").unwrap();
            l.parse::<u64>().unwrap()..=r.parse::<u64>().unwrap()
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
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "};
        assert_eq!(solve(input), 3);
    }
}
