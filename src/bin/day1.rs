use aoc2020::get_input_as_str;

fn get_input_real() -> Vec<u32> {
    get_input_as_str(1)
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part1(inp: &[u32]) -> u32 {
    for (k, i) in inp.iter().enumerate() {
        for j in inp.iter().skip(k + 1) {
            if i + j == 2020 {
                return i * j;
            }
        }
    }

    panic!("No solution found !");
}

fn part2(inp: &[u32]) -> u32 {
    for (m, i) in inp.iter().enumerate() {
        for (n, j) in inp.iter().enumerate().skip(m + 1) {
            for k in inp.iter().skip(n + 1) {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }

    panic!("No solution found !");
}

fn main() {
    let inp = get_input_real();
    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn yolo() {
        let sample = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(&sample), 514579);
        assert_eq!(part2(&sample), 241861950);
    }
}
