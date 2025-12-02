use regex::Regex;

pub fn solve(input: &str) -> u64 {
    let mut result: u64 = 0;
    let part_re = Regex::new(r"(\d+)-(\d+)").unwrap();
    for part in input.split(",") {
        let (_, [a_s, b_s]) = part_re.captures(part).unwrap().extract();
        let a: u64 = a_s.parse().unwrap();
        let b: u64 = b_s.parse().unwrap();
        for i in a..=b {
            let i_s = i.to_string();
            for n in 2..=i_s.len() {
                if i_s.len().is_multiple_of(n) {
                    let chunk_size = i_s.len() / n;
                    let chunks: Vec<_> = i_s.as_bytes().chunks(chunk_size).collect();
                    if chunks.iter().all(|&c| c == chunks[0]) {
                        result += i;
                        break;
                    }
                }
            }

        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(solve(input), 4174379265);
    }
}