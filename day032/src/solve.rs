pub fn solve(input: &str) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        let cs: Vec<u64> = line
            .chars()
            .map(|x| x.to_digit(10).unwrap().into())
            .collect();
        let mut i = 0;
        let mut val = 0;
        for digit in (1..=12).rev() {
            let (i2, x) = find_max(&cs, i, digit);
            i = i2 + 1;
            val = val * 10 + x;
        }
        result += val;
    }
    result
}

fn find_max(cs: &Vec<u64>, ix: usize, digits_left: usize) -> (usize, u64) {
    let (i, &x) = cs[ix..(cs.len() - (digits_left - 1))]
        .iter()
        .enumerate()
        .max_by_key(|&(i, c)| (c, -(i as i64)))
        .unwrap();
    (ix + i, x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};
        assert_eq!(solve(input), 3121910778619);
    }
}
