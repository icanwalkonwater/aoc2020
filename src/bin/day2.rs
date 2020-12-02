use aoc2020::get_input_line_by_line;

type InputTuple = (usize, usize, char, String);

fn parse(inp: impl Iterator<Item = String>) -> impl Iterator<Item = InputTuple> {
    inp.map(|line| {
        // 1-3 a: abcde
        let start = line.chars().take_while(|&c| c != '-').collect::<String>();
        let end = line
            .chars()
            .skip(start.len() + 1)
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        let car = line.chars().nth(start.len() + 1 + end.len() + 1).unwrap();
        let inp = line
            .chars()
            .skip(start.len() + 1 + end.len() + 4)
            .collect::<String>();

        (start.parse().unwrap(), end.parse().unwrap(), car, inp)
    })
}

#[allow(unused)]
fn part1(inp: impl Iterator<Item = InputTuple>) -> usize {
    inp.filter(|(start, end, car, inp)| {
        let count = inp.chars().filter(|c| c == car).count();
        (*start..=*end).contains(&count)
    })
    .count()
}

#[allow(unused)]
fn part2(inp: impl Iterator<Item = InputTuple>) -> usize {
    inp.filter(|(first, second, car, inp)| {
        let has_first = inp.chars().nth(*first - 1).unwrap() == *car;
        let has_second = inp.chars().nth(*second - 1).unwrap() == *car;
        has_first ^ has_second
    })
    .count()
}

fn main() {
    let inp = parse(get_input_line_by_line(2));

    // println!("Part 1: {}", part1(inp));
    println!("Part 2: {}", part2(inp));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1};

    #[test]
    fn ex_p1() {
        assert_eq!(
            part1(
                vec![
                    (1..=3, 'a', "abcde".into()),
                    (1..=3, 'b', "cdefg".into()),
                    (2..=9, 'c', "ccccccccc".into()),
                ]
                .into_iter()
            ),
            2
        );
    }

    #[test]
    fn parse_p1() {
        assert_eq!(
            parse(vec!["1-3 a: abcde".into()].into_iter()).collect::<Vec<_>>(),
            vec![(1..=3, 'a', "abcde".into())]
        )
    }
}
