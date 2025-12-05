use std::cmp::max;
use std::ops::RangeInclusive;

type RangeVec = Vec<RangeInclusive<u64>>;

pub fn solve(input: &str) -> usize {
    let (fresh_part, _) = input.split_once("\n\n").unwrap();
    let mut fresh = parse_fresh(fresh_part);
    fresh.sort_by_key(|range| *range.start());
    let mut fresh_merged: RangeVec = vec![];
    let mut prev = fresh[0].clone();
    for range in fresh.iter().skip(1) {
        if prev.contains(range.start()) {
            prev = *prev.start()..=max(*prev.end(), *range.end());
        } else {
            fresh_merged.push(prev.clone());
            prev = range.clone();
        }
    }
    fresh_merged.push(prev);
    fresh_merged.into_iter().map(|range| range.count()).sum()
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
        assert_eq!(solve(input), 14);
    }
}
