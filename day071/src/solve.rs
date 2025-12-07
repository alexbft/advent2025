pub fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let n = first.len();
    let mut cur_beams = vec![false; n];
    cur_beams[first.find('S').unwrap()] = true;
    let mut splits = 0;
    for line in lines {
        line.chars().enumerate().for_each(|(i, c)| {
            if c == '^' && cur_beams[i] {
                splits += 1;
                cur_beams[i] = false;
                if i > 0 {
                    cur_beams[i - 1] = true;
                }
                if i < n - 1 {
                    cur_beams[i + 1] = true;
                }
            }
        })
    }
    splits
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
        "};
        assert_eq!(solve(input), 21);
    }
}
