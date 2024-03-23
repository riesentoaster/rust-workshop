use rayon::prelude::*;
use std::{collections::HashMap, fmt::Debug};

pub struct Solver {
    precalculated: [u16; 2_usize.pow(16)],
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Ident<'a>(&'a str);

impl<'a> Ident<'a> {
    fn parse_or_lookup(&self, computed: &HashMap<&str, u16>) -> Option<u16> {
        if let Ok(res) = self.0.parse::<u16>() {
            Some(res)
        } else {
            computed.get(self.0).copied()
        }
    }
}

impl<'a> Debug for Ident<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instr<'a> {
    Value(Ident<'a>),
    Not(Ident<'a>),
    And(Ident<'a>, Ident<'a>),
    Or(Ident<'a>, Ident<'a>),
    LShift(Ident<'a>, usize),
    RShift(Ident<'a>, usize),
}

impl Solver {
    pub fn new(wire_instructions: &str) -> Self {
        let instrs = parse_instrs(wire_instructions);
        let mut precalculated = [0; 2_usize.pow(16)];
        precalculated
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, value)| *value = slow_solve(&instrs, i.try_into().unwrap()));
        Self { precalculated }
    }

    pub fn solve(&self, wire_b_value: u16) -> u16 {
        unsafe { *self.precalculated.get_unchecked(wire_b_value as usize) }
    }
}

fn parse_instrs(wire_instructions: &str) -> HashMap<Ident, Instr> {
    wire_instructions
        .split('\n')
        .filter(|e| !e.is_empty())
        .map(|e| {
            let mut it = e.trim().split(" -> ");
            let rhs = it
                .next()
                .unwrap_or_else(|| panic!("{:?}", e))
                .split(' ')
                .collect::<Vec<&str>>();
            let lhs = Ident(it.next().unwrap_or_else(|| panic!("{:?}", e)));
            let rhs_parsed = match rhs.len() {
                1 => Instr::Value(Ident(rhs[0])),
                2 => match rhs[0] {
                    "NOT" => Instr::Not(Ident(rhs[1])),
                    _ => {
                        panic!("illegal instruction: {:?} -> {:?}", rhs, lhs)
                    }
                },
                3 => match rhs[1] {
                    "AND" => Instr::And(Ident(rhs[0]), Ident(rhs[2])),
                    "OR" => Instr::Or(Ident(rhs[0]), Ident(rhs[2])),
                    "LSHIFT" => Instr::LShift(
                        Ident(rhs[0]),
                        rhs[2].parse::<usize>().unwrap_or_else(|_| {
                            panic!("error when parsing {} in instruction {:?}", rhs[2], e)
                        }),
                    ),
                    "RSHIFT" => Instr::RShift(
                        Ident(rhs[0]),
                        rhs[2].parse::<usize>().unwrap_or_else(|_| {
                            panic!("error when parsing {} in instruction {:?}", rhs[2], e)
                        }),
                    ),
                    _ => {
                        panic!("illegal instruction: {:?} -> {:?}", rhs, lhs)
                    }
                },
                _ => {
                    panic!("illegal instruction: {:?} -> {:?}", rhs, lhs)
                }
            };
            (lhs, rhs_parsed)
        })
        .collect()
}

fn slow_solve(instrs: &HashMap<Ident, Instr>, input: u16) -> u16 {
    let mut known_values = HashMap::from([("b", input)]);

    while !known_values.contains_key("a") {
        let mut inserted = false;
        for (Ident(lhs), rhs) in instrs {
            if !known_values.contains_key(lhs) {
                match rhs {
                    Instr::Value(e) => {
                        if let Some(value) = e.parse_or_lookup(&known_values) {
                            known_values.insert(*lhs, value);
                            inserted = true;
                        }
                    }
                    Instr::Not(e) => {
                        if let Some(value) = e.parse_or_lookup(&known_values) {
                            known_values.insert(*lhs, !value);
                            inserted = true;
                        }
                    }
                    Instr::And(a, b) => {
                        if let Some(value_a) = a.parse_or_lookup(&known_values) {
                            if let Some(value_b) = b.parse_or_lookup(&known_values) {
                                known_values.insert(*lhs, value_a & value_b);
                                inserted = true;
                            }
                        }
                    }
                    Instr::Or(a, b) => {
                        if let Some(value_a) = a.parse_or_lookup(&known_values) {
                            if let Some(value_b) = b.parse_or_lookup(&known_values) {
                                known_values.insert(*lhs, value_a | value_b);
                                inserted = true;
                            }
                        }
                    }
                    Instr::LShift(a, b) => {
                        if let Some(value) = a.parse_or_lookup(&known_values) {
                            known_values.insert(*lhs, value << b);
                            inserted = true;
                        }
                    }
                    Instr::RShift(a, b) => {
                        if let Some(value) = a.parse_or_lookup(&known_values) {
                            known_values.insert(*lhs, value >> b);
                            inserted = true;
                        }
                    }
                }
            }
        }
        if !inserted {
            panic!(
                "Infinite loop with\n{:?}\ninstructions:\n{:?}\n\n",
                known_values, instrs
            )
        }
    }
    *known_values.get("a").unwrap_or_else(|| {
        panic!(
            "did not find identifier \"a\" in known values:\n{:?}",
            known_values
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_instructions() {
        const RES: u16 = 0x1337;
        assert_eq!(RES, Solver::new("b -> a").solve(RES));
        assert_eq!(!RES, Solver::new("NOT b -> a").solve(RES));
        assert_eq!(RES, Solver::new("b AND b -> a").solve(RES));
        assert_eq!(0, Solver::new("0 -> z\nb AND z -> a").solve(RES));
        assert_eq!(
            RES,
            Solver::new(&format!("{} -> z\nb AND z -> a", u16::MAX)).solve(RES)
        );
        assert_eq!(RES, Solver::new("b OR b -> a").solve(RES));
        assert_eq!(RES, Solver::new("0 -> z\nb OR z -> a").solve(RES));
        assert_eq!(
            u16::MAX,
            Solver::new(&format!("{} -> z\nb OR z -> a", u16::MAX)).solve(RES)
        );
        for i in 0..16 {
            assert_eq!(
                RES << i,
                Solver::new(&format!("b LSHIFT {} -> a", i)).solve(RES)
            );
            assert_eq!(
                RES >> i,
                Solver::new(&format!("b RSHIFT {} -> a", i)).solve(RES)
            );
        }
    }
}
