pub fn solve(input: &str) -> u64 {
    let mut result = 0;
    for line in input.lines() {
        let cs: Vec<u64> = line
            .chars()
            .map(|x| x.to_digit(10).unwrap().into())
            .collect();
        let (left_i, left) = cs[..cs.len() - 1]
            .iter()
            .enumerate()
            .max_by_key(|&(i, c)| (c, -(i as i64)))
            .unwrap();
        let right = cs[(left_i + 1)..].iter().max().unwrap();
        result += left * 10 + right;
    }
    result
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
        assert_eq!(solve(input), 357);
    }
}
