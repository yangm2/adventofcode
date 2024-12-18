use core::slice;
use std::error::Error;

use anyhow::Result;

#[doc = include_str!("../README.md")]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Rule((u8, u8));

impl Rule {
    fn new(first: u8, second: u8) -> Self {
        Rule { 0: (first, second) }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Rules(Vec<Rule>);

impl Rules {
    fn new() -> Self {
        Rules(vec![])
    }
}

// copied from rustlib/src/rust/library/alloc/src/vec/mod.rs#L3458-L3466
impl<'a> IntoIterator for &'a Rules {
    type Item = &'a Rule;

    type IntoIter = slice::Iter<'a, Rule>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct Update {
    pages: Vec<u8>,
    correct: Option<bool>,
}

impl Update {
    fn new(p: &Vec<u8>) -> Self {
        Update {
            pages: p.clone(),
            correct: None,
        }
    }

    fn middle(&self) -> Result<u8> {
        assert_eq!(is_evenodd(self.pages.len()), EvenOdd::Odd);
        let mid_idx = self.pages.len() / 2;
        Ok(self.pages.get(mid_idx).unwrap().clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Updates(Vec<Update>);

impl Updates {
    fn new() -> Self {
        Updates { 0: vec![] }
    }
}

// copied from rustlib/src/rust/library/alloc/src/vec/mod.rs#L3469-L3476
impl<'a> IntoIterator for &'a mut Updates {
    type Item = &'a mut Update;
    type IntoIter = slice::IterMut<'a, Update>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

fn parse_input(input_txt: &str) -> Result<(Rules, Updates)> {
    let mut result_r = Rules::new();
    let mut result_u = Updates::new();

    for line in input_txt.lines() {
        if line.contains("|") {
            let pages: Vec<u8> = line.split("|").map(|s| s.parse::<u8>().unwrap()).collect();
            assert_eq!(pages.len(), 2);
            result_r.0.push(Rule::new(
                pages.first().unwrap().clone(),
                pages.last().unwrap().clone(),
            ));
        } else if line.contains(",") {
            let pages: Vec<u8> = line.split(",").map(|s| s.parse::<u8>().unwrap()).collect();
            assert!((pages.len() > 1));
            result_u.0.push(Update::new(&pages))
        }
    }

    // not sure if sorting the rules is necessary
    result_r.0.sort();

    Ok((result_r, result_u))
}

fn check_updates(page_ordering_rules: &Rules, update_pages: &mut Updates) -> Result<()> {
    for update in update_pages.into_iter() {
        dbg!(&update);

        let tmp_rules = page_ordering_rules
            .into_iter()
            .filter(|r| update.pages.contains(&r.0.0));
        let relevant_rules = tmp_rules
            .into_iter()
            .filter(|r| update.pages.contains(&r.0.1));

        let mut correct_count = 0;
        for rule in relevant_rules.clone() {
            if update.correct.is_none() {
                let first_index = update.pages.iter().position(|p| *p == rule.0.0).unwrap();
                let second_index = update.pages.iter().position(|p| *p == rule.0.1).unwrap();

                if first_index < second_index {
                    correct_count += 1;
                } else {
                    // disqualify after first rule fails
                    update.correct = Some(false);
                    dbg!(&rule);
                }
            }
        }

        // all of the rules were obeyed
        if correct_count == relevant_rules.count() {
            update.correct = Some(true);
        }
    }

    Ok(())
}

// ------------- MAIN --------------
fn main() -> Result<(), Box<dyn Error>> {
    // compile input file into binary
    const INPUT_TXT: &str = include_str!("../input.txt");
    let (page_ordering_rules, mut update_pages): (Rules, Updates) =
        parse_input(&INPUT_TXT).unwrap();

    // dbg!(page_ordering_rules, update_pages);

    check_updates(&page_ordering_rules, &mut update_pages).unwrap();

    let mut sum_of_middle_page_nums: u32 = 0;
    for (idx, update) in update_pages.0.iter().enumerate() {
        if update.correct.is_some_and(|v| v) {
            sum_of_middle_page_nums += update.middle().unwrap() as u32;
        }
    }

    println!("part 1 sum of middles = {sum_of_middle_page_nums}");

    Ok(())
}

// ------------- TESTS --------------

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

        let (page_ordering_rules, mut update_pages): (Rules, Updates) =
            parse_input(&input_txt).unwrap();

        check_updates(&page_ordering_rules, &mut update_pages).unwrap();

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
