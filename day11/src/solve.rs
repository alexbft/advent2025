pub fn solve(input: &str) -> usize {
    let mut dial = 50;
    let mut zeroes = 0usize;
    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let rot = line[1..].parse::<i32>().unwrap();
        dial = if dir == 'L' { dial - rot } else { dial + rot };
        dial = ((dial % 100) + 100) % 100;
        if dial == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "};
        assert_eq!(solve(input), 3);
    }
}