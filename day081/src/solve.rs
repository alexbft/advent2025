use std::collections::HashMap;
use partial_sort::PartialSort;

pub fn solve(input: &str) -> u64 {
    solve_n(input, 1000)
}

struct Node {
    pub pos: (f64, f64, f64),
    pub color: usize,
}

struct Edge {
    pub from: usize,
    pub to: usize,
    pub distance: f64,
}

fn solve_n(input: &str, n: usize) -> u64 {
    let mut nodes: Vec<Node> = input
        .lines()
        .map(|line| {
            let mut pos = line.split(",");
            let x: f64 = pos.next().unwrap().parse().unwrap();
            let y: f64 = pos.next().unwrap().parse().unwrap();
            let z: f64 = pos.next().unwrap().parse().unwrap();
            Node {
                pos: (x, y, z),
                color: 0,
            }
        })
        .collect();
    let mut edges: Vec<Edge> = Vec::new();
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            edges.push(Edge {
                from: i,
                to: j,
                distance: calc_distance(nodes[i].pos, nodes[j].pos),
            })
        }
    }
    edges.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    let mut next_color = 1;
    for edge in edges.iter().take(n) {
        let a = nodes[edge.from].color;
        let b = nodes[edge.to].color;
        if a == 0 && b == 0 {
            nodes[edge.from].color = next_color;
            nodes[edge.to].color = next_color;
            next_color += 1;
            continue;
        }
        if a == 0 {
            nodes[edge.from].color = b;
            continue;
        }
        if b == 0 {
            nodes[edge.to].color = a;
            continue;
        }
        if a != b {
            for node in &mut nodes {
                if node.color == b {
                    node.color = a;
                }
            }
        }
    }
    let mut color_counter = HashMap::new();
    for node in &nodes {
        if node.color != 0 {
            *color_counter.entry(node.color).or_insert(0u64) += 1;
        }
    }
    let mut sizes: Vec<u64> = color_counter.values().cloned().collect();
    sizes.partial_sort(3, |a, b| b.cmp(a));
    sizes.iter().take(3).product()
}

fn calc_distance(from: (f64, f64, f64), to: (f64, f64, f64)) -> f64 {
    let x = to.0 - from.0;
    let y = to.1 - from.1;
    let z = to.2 - from.2;
    (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689
        "};
        assert_eq!(solve_n(input, 10), 40);
    }
}
