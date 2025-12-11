use itertools::*;

type Point = (u64, u64);

pub fn solve(input: &str) -> u64 {
    let points: Vec<Point> = input.lines().map(parse_point).collect();
    points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| rect_area(a, b))
        .max()
        .unwrap()
}

fn parse_point(line: &str) -> Point {
    let (a, b) = line.split_once(',').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn rect_area(point1: &Point, point2: &Point) -> u64 {
    (point1.0.abs_diff(point2.0) + 1) * (point1.1.abs_diff(point2.1) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3
        "};
        assert_eq!(solve(input), 50);
    }
}
