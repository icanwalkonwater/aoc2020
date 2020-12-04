use aoc2020::get_input_as_str;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Height {
    Cm(u32),
    In(u32),
    Nope,
}

#[derive(Default, Debug, Clone)]
struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn parse(inp: &str) -> Vec<Passport> {
    let mut passports = Vec::new();

    let mut current = Passport::default();
    for line in inp.split("\n") {
        if line.trim().is_empty() {
            passports.push(current.clone());
            current = Passport::default();
            continue;
        }

        for field in line.split_whitespace() {
            let (name, value) = field.trim().split_at(field.find(':').unwrap());
            let value = &value[1..];
            match name {
                "byr" => current.byr = Some(value.parse().unwrap()),
                "iyr" => current.iyr = Some(value.parse().unwrap()),
                "eyr" => current.eyr = Some(value.parse().unwrap()),
                "hgt" => {
                    if value.ends_with("cm") {
                        let val = value[..value.len() - 2].parse().unwrap();
                        current.hgt = Some(Height::Cm(val));
                    } else if value.ends_with("in") {
                        let val = value[..value.len() - 2].parse().unwrap();
                        current.hgt = Some(Height::In(val));
                    } else {
                        current.hgt = Some(Height::Nope);
                    }
                }
                "hcl" => current.hcl = Some(value.to_string()),
                "ecl" => current.ecl = Some(value.to_string()),
                "pid" => current.pid = Some(value.to_string()),
                "cid" => current.cid = Some(value.to_string()),
                _ => panic!("Unknown field {}", field),
            }
        }
    }
    passports.push(current);

    passports
}

fn part1(inp: &[Passport]) -> usize {
    inp.iter()
        .filter(|p| {
            p.byr.is_some()
                && p.iyr.is_some()
                && p.eyr.is_some()
                && p.hgt.is_some()
                && p.hcl.is_some()
                && p.ecl.is_some()
                && p.pid.is_some()
        })
        .count()
}

fn part2(inp: &[Passport]) -> usize {
    inp.iter()
        .filter(|p| {
            let byr = p.byr.is_some() && (1920..=2002).contains(&p.byr.unwrap());
            let iyr = p.iyr.is_some() && (2010..=2020).contains(&p.iyr.unwrap());
            let eyr = p.eyr.is_some() && (2020..=2030).contains(&p.eyr.unwrap());
            let hgt = match p.hgt {
                Some(Height::Cm(x)) if (150..=193).contains(&x) => true,
                Some(Height::In(x)) if (59..=76).contains(&x) => true,
                _ => false,
            };

            let hcl = if let Some(hcl) = &p.hcl {
                hcl.chars().nth(0).unwrap() == '#'
                    && hcl
                        .chars()
                        .skip(1)
                        .all(|c| ('0'..='9').contains(&c) || ('a'..='z').contains(&c))
            } else {
                false
            };

            let ecl = if let Some(ecl) = &p.ecl {
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl.as_str())
            } else {
                false
            };

            let pid = p.pid.is_some()
                && p.pid.as_ref().unwrap().len() == 9
                && p.pid.as_ref().unwrap().chars().all(|c| c.is_numeric());

            byr && iyr && eyr && hgt && hcl && ecl && pid
        })
        .count()
}

fn main() {
    let passports = parse(&get_input_as_str(4));
    println!("Part 1: {}", part1(&passports));
    println!("Part 2: {}", part2(&passports));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part1, Height};

    #[test]
    fn parse_ex() {
        let inp = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in";

        let passports = parse(inp);
        assert_eq!(passports.len(), 4);
        assert_eq!(passports[0].ecl.as_ref().unwrap(), "gry");
        assert_eq!(passports[0].pid.as_ref().unwrap(), "860033327");
        assert_eq!(passports[0].eyr.unwrap(), 2020);
        assert_eq!(passports[0].hcl.as_ref().unwrap(), "#fffffd");
        assert_eq!(passports[0].byr.unwrap(), 1937);
        assert_eq!(passports[0].iyr.unwrap(), 2017);
        assert_eq!(passports[0].cid.as_ref().unwrap(), "147");
        assert_eq!(passports[0].hgt.unwrap(), Height::Cm(183));
    }

    #[test]
    fn part1_ex() {
        let inp = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in";

        let passports = parse(inp);

        assert_eq!(passports.len(), 4);
        assert_eq!(part1(&passports), 2);
    }
}
