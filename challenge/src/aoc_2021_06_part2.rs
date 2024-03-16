use rand::Rng;

const FIXED_VALUES: [usize; 9] = [1421, 1401, 1191, 1154, 1034, 950, 905, 779, 768];

pub fn aoc_2021_06_part2(input: &str) -> usize {
    aoc_2021_06_part2_parse_manual(input)
        .iter()
        .zip(FIXED_VALUES)
        .map(|(l, r)| l * r)
        .sum()
}

pub fn aoc_2021_06_part2_slow_parse(input: &str) -> usize {
    aoc_2021_06_part2_parse(input)
        .iter()
        .zip(FIXED_VALUES)
        .map(|(l, r)| l * r)
        .sum()
}

pub fn aoc_2021_06_part2_parse(input: &str) -> [usize; 9] {
    let mut out = [0; 9];
    input
        .split(',')
        .map(|e| e.parse::<usize>().unwrap())
        .for_each(|e| out[e] += 1);
    out
}

const ZERO: u8 = b'0';

pub fn aoc_2021_06_part2_parse_manual(input: &str) -> [usize; 9] {
    let mut out = [0; 9];
    input
        .chars()
        .step_by(2)
        .map(|e| e as u8)
        .map(|e| e - ZERO)
        .map(|e| e as usize)
        .for_each(|e| out[e] += 1);
    out
}

pub fn aoc_2021_06_part2_naive(input: &str) -> usize {
    let input = aoc_2021_06_part2_parse(input);
    let mut res = 0;
    for (i, k) in input.iter().enumerate() {
        let mut entries = vec![i];
        for _day in 0..80 {
            entries = entries
                .iter()
                .cloned()
                .flat_map(|e| if e == 0 { vec![6, 8] } else { vec![e - 1] })
                .collect();
        }
        res += entries.len() * k
    }
    res
}

pub fn aoc_2021_06_part2_generate_str(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| rng.gen_range(0_u8..=8_u8))
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::prelude::*;
    const TEST_LENGTHS: [usize; 4] = [1, 3, 10, 50];

    #[test]
    fn example() {
        let str = "3,4,3,1,2";
        let expected = 5934;
        assert_eq!(aoc_2021_06_part2(str), expected);
        assert_eq!(aoc_2021_06_part2_slow_parse(str), expected);
        assert_eq!(aoc_2021_06_part2_naive(str), expected);
    }

    #[test]
    fn impls_are_equal() {
        TEST_LENGTHS.into_par_iter().for_each(|len| {
            (0..(1000 / len)).into_par_iter().for_each(|_| {
                let string = aoc_2021_06_part2_generate_str(len);
                let fastest = aoc_2021_06_part2(&string);
                let medium = aoc_2021_06_part2_slow_parse(&string);
                let naive = aoc_2021_06_part2_naive(&string);
                assert_eq!(fastest, medium, "{}: {}, {}", string, fastest, medium);
                assert_eq!(fastest, naive, "{}: {}, {}", string, fastest, naive);
            });
        });
    }
    #[test]
    fn automated_and_manual_parse_are_equal() {
        TEST_LENGTHS.into_par_iter().for_each(|len| {
            (0..(1000 / len)).into_par_iter().for_each(|_| {
                let string = aoc_2021_06_part2_generate_str(len);
                assert_eq!(
                    aoc_2021_06_part2_parse(&string),
                    aoc_2021_06_part2_parse_manual(&string),
                    "{}: {:?}, {:?}",
                    string,
                    aoc_2021_06_part2_parse(&string),
                    aoc_2021_06_part2_parse_manual(&string)
                )
            });
        });
    }
}
