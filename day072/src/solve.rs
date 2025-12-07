pub fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let first = lines.next().unwrap();
    let n = first.len();
    let mut cur_beams = vec![0; n];
    cur_beams[first.find('S').unwrap()] = 1;
    for line in lines {
        line.chars().enumerate().for_each(|(i, c)| {
            if c == '^' && cur_beams[i] > 0 {
                let cur = cur_beams[i];
                cur_beams[i] = 0;
                if i > 0 {
                    cur_beams[i - 1] += cur
                }
                if i < n - 1 {
                    cur_beams[i + 1] += cur
                }
            }
        });
    }
    cur_beams.iter().sum()
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
        assert_eq!(solve(input), 40);
    }
}
