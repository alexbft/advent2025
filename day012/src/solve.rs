pub fn solve(input: &str) -> usize {
    let mut dial = 50;
    let mut zeroes = 0usize;
    for line in input.lines() {
        let dir = line.chars().next().unwrap();
        let mut rot = line[1..].parse::<usize>().unwrap();
        zeroes += rot / 100;
        rot %= 100;
        let prev_dial = dial;
        dial = if dir == 'L' { dial - rot as i32 } else { dial + rot as i32 };
        let new_dial = ((dial % 100) + 100) % 100;
        if prev_dial != 0 && (new_dial == 0 || dial != new_dial) {
            zeroes += 1;
        }
        dial = new_dial;
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
            R201
        "};
        assert_eq!(solve(input), 8);
    }
}