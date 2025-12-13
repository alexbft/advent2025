use itertools::Itertools;
use std::cmp::{max, min};

type Point = (u64, u64);

pub fn solve(input: &str) -> u64 {
    let mut points: Vec<Point> = input.lines().map(parse_point).collect();
    points.push(points[0].clone());
    let mut candidates = vec![];
    for (a, b) in points.iter().tuple_combinations() {
        let x0 = min(a.0, b.0);
        let x1 = max(a.0, b.0);
        let y0 = min(a.1, b.1);
        let y1 = max(a.1, b.1);
        if x0 == x1 || y0 == y1 {
            continue;
        }
        let top_left = (x0, y0);
        let bottom_right = (x1, y1);
        let mut is_intersection = false;
        for (a, b) in points.iter().tuple_windows() {
            if line_intersects_rect(a, b, &top_left, &bottom_right) {
                is_intersection = true;
                break;
            }
        }
        if !is_intersection {
            candidates.push((top_left, bottom_right))
        }
    }
    //println!("candidates: {:?}", candidates);
    candidates
        .iter()
        .map(|(a, b)| rect_area(a, b))
        .max()
        .unwrap()
}

fn parse_point(line: &str) -> Point {
    let (a, b) = line.split_once(',').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

fn line_intersects_rect(p0: &Point, p1: &Point, top_left: &Point, bottom_right: &Point) -> bool {
    let (x0, y0) = *top_left;
    let (x1, y1) = *bottom_right;

    if p0.0 == p1.0 {                      
        let lx = p0.0;
        if lx <= x0 || lx >= x1 {
            return false;
        }
        let ly0 = min(p0.1, p1.1);
        let ly1 = max(p0.1, p1.1);
        if ly1 <= y0 || ly0 >= y1 {
            return false;
        }
        true
    } else {
        let ly = p0.1;
        if ly <= y0 || ly >= y1 {
            return false;
        }
        let lx0 = min(p0.0, p1.0);
        let lx1 = max(p0.0, p1.0);
        if lx1 <= x0 || lx0 >= x1 {
            return false;
        }
        true
    }
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
        assert_eq!(solve(input), 24);
    }
}
