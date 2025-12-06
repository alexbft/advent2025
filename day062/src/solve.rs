pub fn solve(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let ops_str = *lines.last().unwrap();
    let ops = ops_str.split_ascii_whitespace().collect::<Vec<_>>();
    let data: Vec<Vec<char>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    let exercise_lines: Vec<String> = transpose(data)
        .iter()
        .map(|cs| {
            let line: String = cs.into_iter().collect();
            line.trim().to_owned()
        })
        .collect();
    let exercises: Vec<_> = exercise_lines.split(|line| line.is_empty()).collect();
    let exercises_with_ops = exercises.into_iter().zip(ops.into_iter());
    let mut result_sum = 0;
    for (args, op) in exercises_with_ops {
        let args_num = args.iter().map(|x| x.parse::<u64>().unwrap());
        // println!("{:?} {:?}", op, args_num);
        let result: u64 = match op {
            "+" => args_num.sum(),
            "*" => args_num.product(),
            _ => panic!(),
        };
        result_sum += result;
    }
    result_sum
}

fn transpose(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![];
    let max_w = data.iter().map(|line| line.len()).max().unwrap();
    for x in 0..max_w {
        let mut col = vec![];
        for y in 0..data.len() {
            let c = if x < data[y].len() { data[y][x] } else { ' ' };
            col.push(c);
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
        assert_eq!(solve(input), 3263827);
    }
}
