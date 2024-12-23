use anyhow::Result;
use itertools::Itertools;
use std::{error::Error, fmt::Debug};
use memoize::memoize;

/// missing operators allowed in the equation
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

#[inline(always)]
#[memoize]
fn memoized_mul(lhs: u64, rhs: u64) -> u64 {
    lhs.checked_mul(rhs).unwrap()
}

#[inline]
#[memoize]
fn concat_u64(lhs: u64, rhs: u64) -> u64 {
    format!("{lhs}{rhs}").parse::<u64>().unwrap()
}

/// Ugly memoization function to memoize the permutations with replacement; not lazy
#[inline]
#[memoize]
fn memoize_permutations_with_replacment(operator_set: Vec<Operator>, oplen: usize) -> Vec<Vec<Operator>> {
    itertools::repeat_n(operator_set, oplen).multi_cartesian_product().collect()
}

impl Operator {
    // FIXME: learn how to manually memoize this method
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Multiply => memoized_mul(lhs, rhs),
            Self::Concatenate => concat_u64(lhs, rhs),
        }
    }
}

/// represent an equation with a simple LHS and list of operands on the RHS
#[derive(Clone, Debug)]
struct Equation {
    lhs: u64,
    rhs: Vec<u64>,
}

/// EquationSolver is a struct that can solve an equation
#[derive(Clone, Debug)]
struct EquationSolver {
    eqn: Equation,
    operands: Vec<u64>,
}

impl EquationSolver {
    fn new(eqn: Equation) -> Self {
        Self {
            eqn: eqn.clone(),
            operands: eqn.rhs.clone(),
        }
    }

    /// helper_solve_rec is a recursive helper function that evalutes the equation with the given operators
    /// FIXME: too much cloning ... could probably use a more functional programming recursive pattern
    fn helper_solve_rec(&self, combo: Option<Vec<Operator>>) -> Option<u64> {
        if self.operands.len() == 1 {
            return Some(self.operands[0]);
        }

        // mutable local copies on the recursion stack
        let mut my_self = self.clone();
        let mut my_combos = combo.clone().unwrap();

        let tail_operand = my_self.operands.pop().unwrap();
        let tail_operator = my_combos.pop().unwrap();

        let val = my_self.helper_solve_rec(Some(my_combos));

        // optimization: recursed evaluation already short-circuited, so we can short-circuit
        if val.is_none() {
            return None;
        }

        let result = tail_operator.apply(val.unwrap(), tail_operand);

        // optimization: if the result is greater than the LHS, then we can short-circuit
        if result > self.eqn.lhs {
            return None;
        }

        // println!(
        //     "    {} {:?} {} = {}",
        //     val.unwrap(),
        //     tail_operator,
        //     tail_operand,
        //     result
        // );
        Some(result)
    }

    /// finds the operators (if any) for the equation
    fn solve(&self, operator_set: Vec<Operator>) -> Result<()> {
        let oplen = self.eqn.rhs.len() - 1;
        // dbg!(oplen);

        // generate all possible permutations (with replacement!!!) of the operators
        let ordered_combos = memoize_permutations_with_replacment(operator_set, oplen);

        // println!(
        //     "Looking for {:?} from {} combos",
        //     &self.eqn,
        //     ordered_combos.clone().count()
        // );

        for combo in ordered_combos {
            // dbg!(&combo);

            let eval = self.helper_solve_rec(Some(combo.clone()));

            if eval.is_some() && self.eqn.lhs == eval.unwrap() {
                println!(
                    "Found a solution: {:?} for {:?}",
                    combo,
                    self.clone().eqn.rhs
                );
                return Ok(());
            }
        }

        Err(anyhow::anyhow!("No solution found"))
    }
}

#[derive(Clone, Debug)]
struct Equations(Vec<Equation>);

impl Equations {
    /// parsing (came from CoPilot)
    fn from_str(input_txt: &str) -> Self {
        let mut eqns = Vec::<Equation>::new();
        for line in input_txt.lines() {
            let mut parts = line.split(": ");
            let lhs = parts.next().unwrap().parse::<u64>().unwrap();
            let rhs = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let eqn = Equation { lhs, rhs };
            eqns.push(eqn);
        }

        Self(eqns)
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_1() {
        const FINAL_ANSWER: usize = 3749;

        let tmp: String = String::from(
            "190: 10 19\n\
             3267: 81 40 27\n\
             83: 17 5\n\
             156: 15 6\n\
             7290: 6 8 6 15\n\
             161011: 16 10 13\n\
             192: 17 8 14\n\
             21037: 9 7 18 13\n\
             292: 11 6 16 20",
        );
        let input_txt = tmp.as_str();
        let eqns = Equations::from_str(input_txt);

        let mut count_possibly_true_equations = 0;
        for eqn in eqns.0 {
            let solver = EquationSolver::new(eqn.clone());
            // dbg!(&solver);

            if solver
                .solve(vec![Operator::Add, Operator::Multiply])
                .is_ok()
            {
                count_possibly_true_equations += eqn.lhs;
            }
        }

        assert_eq!(FINAL_ANSWER, count_possibly_true_equations as usize);
    }

    #[test]
    fn test_2() {
        const FINAL_ANSWER: usize = 11387;

        let tmp: String = String::from(
            "190: 10 19\n\
             3267: 81 40 27\n\
             83: 17 5\n\
             156: 15 6\n\
             7290: 6 8 6 15\n\
             161011: 16 10 13\n\
             192: 17 8 14\n\
             21037: 9 7 18 13\n\
             292: 11 6 16 20",
        );
        let input_txt = tmp.as_str();
        let eqns = Equations::from_str(input_txt);

        let mut count_possibly_true_equations = 0;
        for eqn in eqns.0 {
            let solver = EquationSolver::new(eqn.clone());
            // dbg!(&solver);

            if solver
                .solve(vec![
                    Operator::Add,
                    Operator::Multiply,
                    Operator::Concatenate,
                ])
                .is_ok()
            {
                count_possibly_true_equations += eqn.lhs;
            }
        }

        assert_eq!(FINAL_ANSWER, count_possibly_true_equations as usize);
    }
}

#[doc = include_str!("../README.md")]
fn main() -> Result<(), Box<dyn Error>> {
    const INPUT_TXT: &str = include_str!("../input.txt");

    let eqns = Equations::from_str(INPUT_TXT);

    let mut part1_total_calibration_result = 0;
    let mut unsolved_eqns = Vec::<Equation>::new();
    for eqn in eqns.0 {
        let solver = EquationSolver::new(eqn.clone());
        // dbg!(&solver);

        let solve = solver.solve(vec![Operator::Add, Operator::Multiply]);
        if solve.is_ok() {
            part1_total_calibration_result += eqn.lhs;
        } else {
            unsolved_eqns.push(eqn);
        }
    }

    println!(
        "Part 1 Count of possibly true equations: {}",
        part1_total_calibration_result
    );

    let mut part2_total_calibration_result = part1_total_calibration_result;
    for eqn in unsolved_eqns {
        let solver = EquationSolver::new(eqn.clone());
        // dbg!(&solver);

        let solve = solver.solve(vec![
            Operator::Add,
            Operator::Multiply,
            Operator::Concatenate,
        ]);
        if solve.is_ok() {
            part2_total_calibration_result += eqn.lhs;
        } else {
            // dbg!(solve, eqn);
        }
    }

    println!(
        "Part2 Count of possibly true equations: {}",
        part2_total_calibration_result
    );

    Ok(())
}
