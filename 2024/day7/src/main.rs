use anyhow::Result;
use itertools::Itertools;
use std::error::Error;

/// missing operators allowed in the equation
#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Multiply => lhs.checked_mul(rhs).unwrap(),
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

        println!(
            "    {} {:?} {} = {}",
            val.unwrap(),
            tail_operator,
            tail_operand,
            result
        );
        Some(result)
    }

    /// finds the operators (if any) for the equation
    fn solve(&self) -> Result<()> {
        let oplen = self.eqn.rhs.len() - 1;
        // dbg!(oplen);

        // generate all possible permutations (with replacement!!!) of the operators
        let ordered_combos = itertools::repeat_n([Operator::Add, Operator::Multiply], oplen)
            .multi_cartesian_product();

        println!(
            "Looking for {:?} from {} combos",
            &self.eqn,
            ordered_combos.clone().count()
        );

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

            if solver.solve().is_ok() {
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
    for eqn in eqns.0 {
        let solver = EquationSolver::new(eqn.clone());
        // dbg!(&solver);

        let solve = solver.solve();
        if solve.is_ok() {
            part1_total_calibration_result += eqn.lhs;
        } else {
            // dbg!(solve, eqn);
        }
    }

    println!(
        "Count of possibly true equations: {}",
        part1_total_calibration_result
    );

    Ok(())
}