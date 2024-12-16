use std::error::Error;

use anyhow::Result;

#[doc = include_str!("../README.md")]

struct Rule((u8, u8));

impl Rule {
    fn new(first: u8, second: u8) -> Self {
        Rule((first, second))
    }
}

struct Rules(Vec<Rule>);

impl Rules {
    fn new() -> Self {
        Rules(vec![])
    }
}

struct Update {
    pages: Vec<u8>,
    correct: Option<bool>,
}

#[derive(Debug, Eq, PartialEq)]
enum EvenOdd {
    Even,
    Odd,
}

fn is_evenodd(v: usize) -> EvenOdd {
    if let 0 = v % 2 {
        EvenOdd::Even
    } else {
        EvenOdd::Odd
    }
}

impl Update {
    fn middle(&self) -> Result<u8> {
        assert_eq!(is_evenodd(self.pages.len()), EvenOdd::Odd);
        let mid_idx = self.pages.len() / 2;
        Ok(self.pages.get(mid_idx).unwrap().clone())
    }
}

struct Updates(Vec<Update>);

impl Updates {
    fn new() -> Self {
        Updates(vec![])
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // compile input file into binary
    const INPUT_TXT: &str = include_str!("../input.txt");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test1() {
        const PARTIAL_ANSWER: [u8; 3] = [61, 53, 29];
        const FINAL_ANSWER: u8 = 143;

        let input_txt: &str = "47|53\n\
                               97|13\n\
                               97|61\n\
                               97|47\n\
                               75|29\n\
                               61|13\n\
                               75|53\n\
                               29|13\n\
                               97|29\n\
                               53|29\n\
                               61|53\n\
                               97|53\n\
                               61|29\n\
                               47|13\n\
                               75|47\n\
                               97|75\n\
                               47|61\n\
                               75|61\n\
                               47|29\n\
                               75|13\n\
                               53|13\n\
                               \n\
                               75,47,61,53,29\n\
                               97,61,53,29,13\n\
                               75,29,13\n\
                               75,97,47,61,53\n\
                               61,13,29\n\
                               97,13,75,29,47";

        let mut page_ordering_rules: Rules = Rules::new();
        let mut update_pages: Updates = Updates::new();

        let mut sum_of_middle_page_nums = 0;
        for (idx, update) in update_pages.0.iter().enumerate() {
            if update.correct.is_some_and(|v| v) {
                assert_eq!(PARTIAL_ANSWER[idx], update.middle().unwrap());
                sum_of_middle_page_nums += update.middle().unwrap();
            }
        }
        assert_eq!(FINAL_ANSWER, sum_of_middle_page_nums);
    }
}
