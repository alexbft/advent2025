use char_matrix::CharMatrix;

pub fn solve(input: &str) -> usize {
    let matrix = CharMatrix::new(input);
    let mut count = 0;
    for y in 0..matrix.height() {
        for x in 0..matrix.width() {
            if matrix.get(x, y) == '@' && adjacent_count(&matrix, x, y) < 4 {
                count += 1
            }
        }
    }
    count
}

fn adjacent_count(matrix: &CharMatrix, x: i32, y: i32) -> u32 {
    let mut count = 0;
    for yy in (y - 1)..=(y + 1) {
        for xx in (x - 1)..=(x + 1) {
            if xx == x && yy == y {
                continue;
            }
            if matrix.safe_get(xx, yy) == Some('@') {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "};
        assert_eq!(solve(input), 13);
    }
}
