use aoc2020::get_input_line_by_line;

fn parse(inp: impl Iterator<Item = String>) -> Vec<Vec<bool>> {
    inp.map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

fn descent(inp: &[Vec<bool>], step_right: usize, step_down: usize) -> usize {
    inp.iter()
        .enumerate()
        .step_by(step_down)
        .filter(|(i, line)| *line.iter().cycle().nth(i * step_right / step_down).unwrap())
        .count()
}

fn part1(inp: &[Vec<bool>]) -> usize {
    descent(inp, 3, 1)
}

fn part2(inp: &[Vec<bool>]) -> usize {
    let one_one = descent(inp, 1, 1);
    let three_one = descent(inp, 3, 1);
    let five_one = descent(inp, 5, 1);
    let seven_one = descent(inp, 7, 1);
    let one_two = descent(inp, 1, 2);

    one_one * three_one * five_one * seven_one * one_two
}

fn main() {
    let inp = parse(get_input_line_by_line(3));
    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}

#[cfg(test)]
mod tests {
    use crate::{descent, parse};

    #[test]
    fn test_parse() {
        let map = parse(vec![".#".into(), "#.".into()].into_iter());
        assert_eq!(map[0], vec![false, true]);
        assert_eq!(map[1], vec![true, false]);
    }

    #[test]
    fn test_ex() {
        let map = parse(
            r"..##.......
                #...#...#..
                .#....#..#.
                ..#.#...#.#
                .#...##..#.
                ..#.##.....
                .#.#.#....#
                .#........#
                #.##...#...
                #...##....#
                .#..#...#.#"
                .split("\n")
                .map(|s| s.trim().to_string()),
        );

        assert_eq!(descent(&map, 1, 1), 2);
        assert_eq!(descent(&map, 3, 1), 7);
        assert_eq!(descent(&map, 5, 1), 3);
        assert_eq!(descent(&map, 7, 1), 4);
        assert_eq!(descent(&map, 1, 2), 2);
    }
}
