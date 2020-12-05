use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn from_str(input: String) -> Self {
        // let t = input.split(",").collect::<Vec<_>>();
        let reformatted = input
            .split(" ")
            .map(|s| s.split(":").collect::<Vec<_>>())
            .map(|f| format!("{}=\"{}\"", f[0], f[1]))
            .collect::<Vec<_>>()
            .join("\n");
        toml::from_str(&reformatted[..]).unwrap()
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_strict(&self) -> bool {
        self.is_byr_valid()
            && self.is_iyr_valid()
            && self.is_eyr_valid()
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.is_ecl_valid()
            && self.is_pid_valid()
    }

    fn is_byr_valid(&self) -> bool {
        match self.byr.clone() {
            Some(byr) => {
                byr.len() == 4
                    && match byr.parse::<i32>() {
                        Ok(year) => 1920 <= year && year <= 2002,
                        Err(_) => false,
                    }
            }
            None => false,
        }
    }

    fn is_iyr_valid(&self) -> bool {
        match self.iyr.clone() {
            Some(iyr) => {
                iyr.len() == 4
                    && match iyr.parse::<i32>() {
                        Ok(year) => 2010 <= year && year <= 2020,
                        Err(_) => false,
                    }
            }
            None => false,
        }
    }

    fn is_eyr_valid(&self) -> bool {
        match self.eyr.clone() {
            Some(eyr) => {
                eyr.len() == 4
                    && match eyr.parse::<i32>() {
                        Ok(year) => 2020 <= year && year <= 2030,
                        Err(_) => false,
                    }
            }
            None => false,
        }
    }

    fn is_hgt_valid(&self) -> bool {
        match self.hgt.clone() {
            Some(eyr) => {
                eyr.ends_with("cm")
                    && match eyr[..eyr.len() - 2].parse::<i32>() {
                        Ok(height) => 150 <= height && height <= 193,
                        Err(_) => false,
                    }
                    || eyr.ends_with("in")
                        && match eyr[..eyr.len() - 2].parse::<i32>() {
                            Ok(height) => 59 <= height && height <= 76,
                            Err(_) => false,
                        }
            }
            None => false,
        }
    }

    fn is_hcl_valid(&self) -> bool {
        match self.hcl.clone() {
            Some(hcl) => {
                hcl.starts_with("#")
                    && (hcl[1..].chars().all(|c| {
                        (0..=9)
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .contains(&c.to_string())
                    } || ('a'..='f').contains(&c)))
            }
            None => false,
        }
    }

    fn is_ecl_valid(&self) -> bool {
        match self.ecl.clone() {
            Some(ecl) => match ecl.as_str() {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false,
            },
            None => false,
        }
    }

    fn is_pid_valid(&self) -> bool {
        match self.pid.clone() {
            Some(pid) => pid.len() == 9 && pid.chars().all(|c| ('0'..='9').contains(&c)),
            None => false,
        }
    }
}

#[aoc_generator(day4)]
fn generator_input(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|s| s.replace("\n", " "))
        .map(|s| Passport::from_str(s))
        // .map(|f| toml::from_str::<Passport>(&f[..]).unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day4, part1)]
fn part1(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_valid()).count()
}

#[aoc(day4, part2)]
fn part2(input: &[Passport]) -> usize {
    input.iter().filter(|p| p.is_valid_strict()).count()
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2, Passport};
    static INPUT_RAW: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

    #[test]
    fn generator() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(input.len(), 4);
    }

    #[test]
    fn is_valid_strict_detects_valid() {
        let valids: Vec<&str> = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f",
            "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ];
        for valid in valids {
            let passport = Passport::from_str(valid.to_string());
            assert!(passport.is_valid_strict());
        }
    }

    #[test]
    fn is_valid_strict_detects_invalid() {
        let invalids: Vec<&str> = vec![
            "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946",
            "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007",
        ];
        for invalid in invalids {
            let passport = Passport::from_str(invalid.to_string());
            assert!(!passport.is_valid_strict());
        }
    }

    #[test]
    fn day3_part1() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn day3_part2() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(part2(&input), 2);
    }
}
