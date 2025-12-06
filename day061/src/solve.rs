pub fn solve(input: &str) -> u64 {
    let data: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();
    let exercises = transpose(data);
    let mut result_sum = 0;
    for exercise in exercises {
        let op = *exercise.last().unwrap();
        let args = exercise[..exercise.len() - 1]
            .iter()
            .map(|x| x.parse::<u64>().unwrap());
        let result: u64 = match op {
            "+" => args.sum(),
            "*" => args.product(),
            _ => panic!(),
        };
        result_sum += result;
    }
    result_sum
}

fn transpose(data: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    let mut result = vec![];
    for x in 0..data[0].len() {
        let mut col = vec![];
        for y in 0..data.len() {
            col.push(data[y][x]);
        }
        result.push(col);
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
           123 328  51 64
            45 64  387 23
             6 98  215 314
           *   +   *   +
        "};
        assert_eq!(solve(input), 4277556);
    }
}
