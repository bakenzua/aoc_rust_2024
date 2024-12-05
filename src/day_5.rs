#![allow(unused_variables)]

use std::fs::read_to_string;

const EXAMPLE_FILEPATH: &str = "./data/example_5.txt";
const INPUT_FILEPATH: &str = "./data/input_5.txt";

// main entry point for day 5
pub fn run(part: i16) {
    // part 1
    match part {
        1 => {
            let example_result: i32 = part_1(EXAMPLE_FILEPATH);
            println!("Example result: {example_result}");

            let question_result: i32 = part_1(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        2 => {
            let example_result: i32 = part_2(EXAMPLE_FILEPATH);
            println!("Example result: {example_result}");

            let question_result: i32 = part_2(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        _ => {
            panic!("Invalid part specified for day 5")
        }
    }
}

fn part_1(file_path: &str) -> i32 {
    let (rules, manual_updates) = parse_file(file_path);

    let mut result: i32 = 0;

    'update_loop: for man_update in &manual_updates {
        'rule_loop: for rule in &rules {
            match update_rule_check(man_update, rule) {
                RuleCheckResult::DoesNotApply => continue 'rule_loop,
                RuleCheckResult::Fail => continue 'update_loop,
                RuleCheckResult::Pass => continue 'rule_loop,
            }
        }
        // no rules broken
        // add middle element of manual update to result
        let num = (man_update.len() - 1) / 2;
        result += man_update[num];
    }

    result
}

fn part_2(file_path: &str) -> i32 {
    let (rules, manual_updates) = parse_file(file_path);

    let mut updates_failing_rule_check: Vec<Vec<i32>> = Vec::new();

    // find rule breaking updates
    'update_loop: for man_update in &manual_updates {
        'rule_loop: for rule in &rules {
            match update_rule_check(man_update, rule) {
                RuleCheckResult::DoesNotApply => continue 'rule_loop,
                RuleCheckResult::Fail => {
                    updates_failing_rule_check.push(man_update.clone());
                    continue 'update_loop;
                }
                RuleCheckResult::Pass => continue 'rule_loop,
            }
        }
    }

    // TODO fix the bin fire below

    // mutate rule breaking updates
    // println!("{:?}", updates_failing_rule_check);
    // let mut any_fails = true;
    // let mut count = 1;
    for man_update in updates_failing_rule_check.iter_mut() {
        // while any_fails {
        //     any_fails = apply_rules_to(man_update, rules.clone());
        //     print!("Count: {}", count);
        //     println!("{:?}", man_update);
        //     count +=1;
        // }
        // why the above does not work - loop until no fails ¯\_(ツ)_/¯

        // instead the below ugly - keep applying rules until all updates are ordered correctly

        _ = apply_rules_to(man_update, rules.clone());
        _ = apply_rules_to(man_update, rules.clone());
        _ = apply_rules_to(man_update, rules.clone());
    }
    // println!("{:?}", updates_failing_rule_check);
    let result = updates_failing_rule_check
        .iter()
        .map(|x| x[(x.len() - 1) / 2])
        .sum();
    result
}

fn apply_rules_to(update: &mut Vec<i32>, rules: Vec<(i32, i32)>) -> bool {
    let mut any_fails = false;

    for rule in rules {
        match update_rule_check(update, &rule) {
            RuleCheckResult::DoesNotApply => continue,
            RuleCheckResult::Pass => continue,
            RuleCheckResult::Fail => {
                let index_1 = match update.iter().position(|n| n == &rule.0) {
                    Some(n) => n,
                    None => panic!("Rule: {:?} index 1 not found!", &rule.0),
                };
                let index_2 = match update.iter().position(|n| n == &rule.1) {
                    Some(n) => n,
                    None => panic!("Rule: {:?} index 1 not found!", &rule.0),
                };
                // update.swap(index_1, index_2);
                let tmp_val = update.remove(index_2);
                update.insert(index_1, tmp_val);
                any_fails = true;
            }
        }
    }
    any_fails
}

enum ParserState {
    Rules,
    Pages,
}

// parse file in to a vector of tuple rules and vector of safety manual update page vectors
fn parse_file(file_path: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let filetxt = read_to_string(file_path).expect("Error reading file: {file_path}");

    let mut state: ParserState = ParserState::Rules;
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut manual_updates: Vec<Vec<i32>> = Vec::new();

    for line in filetxt.lines() {
        if line == "" {
            state = ParserState::Pages;
        } else {
            match state {
                ParserState::Rules => {
                    let mut rule = line.split('|');
                    let lhs = rule
                        .next()
                        .expect("LHS of rule not found: {line}")
                        .parse::<i32>()
                        .expect("LHS of rule would not parse {line}");
                    let rhs = rule
                        .next()
                        .expect("RHS of rule not found: {line}")
                        .parse::<i32>()
                        .expect("RHS of rule would not parse {line}");
                    rules.push((lhs, rhs));
                }
                ParserState::Pages => {
                    let pages: Vec<i32> = line
                        .split(',')
                        .map(|x| x.parse::<i32>().expect("Could not parse page number: {x}"))
                        .collect();
                    manual_updates.push(pages)
                }
            }
        }
    }
    (rules, manual_updates)
}

enum RuleCheckResult {
    DoesNotApply,
    Pass,
    Fail,
}

fn update_rule_check(man_update: &Vec<i32>, rule: &(i32, i32)) -> RuleCheckResult {
    // get index of rule element 0, continue to next rule if not found
    let index1 = match man_update.iter().position(|n| n == &rule.0) {
        Some(n) => n,
        None => return RuleCheckResult::DoesNotApply,
    };

    // get index of rule element 1, continue to next rule if not found
    let index2 = match man_update.iter().position(|n| n == &rule.1) {
        Some(n) => n,
        None => return RuleCheckResult::DoesNotApply,
    };
    // if rule broken continue to next manual update
    if index1 > index2 {
        return RuleCheckResult::Fail;
    } else {
        return RuleCheckResult::Pass;
    }
}
///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(143, part_1(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(123, part_2(EXAMPLE_FILEPATH));
        assert_eq!(4407, part_2(INPUT_FILEPATH));
    }

    #[test]
    fn test_parser() {
        let (rules, manual_updates) = parse_file(EXAMPLE_FILEPATH);

        assert_eq!(rules[0], (47, 53));

        assert_eq!(manual_updates[0], vec![75, 47, 61, 53, 29]);
    }

    #[test]
    fn test_apply_rules_to_simple() {
        // apply_rules_to(update: &mut Vec<i32>, rules: Vec<(i32, i32)>)
        let mut man_update = vec![1, 2, 3, 4, 5];
        let rule = vec![(3, 2)];
        apply_rules_to(&mut man_update, rule);
        assert_eq!(man_update, vec![1, 3, 2, 4, 5]);
    }

    #[test]
    fn test_apply_rules_to_examples() {
        let (rules, manual_updates) = parse_file(EXAMPLE_FILEPATH);

        // example 1
        let mut man_update = vec![75, 97, 47, 61, 53];
        apply_rules_to(&mut man_update, rules.clone());
        assert_eq!(man_update, vec![97, 75, 47, 61, 53]);

        // example 2
        man_update = vec![61, 13, 29];
        apply_rules_to(&mut man_update, rules.clone());
        assert_eq!(man_update, vec![61, 29, 13]);

        // example 3
        man_update = vec![97, 13, 75, 29, 47];
        apply_rules_to(&mut man_update, rules.clone());
        apply_rules_to(&mut man_update, rules.clone());
        apply_rules_to(&mut man_update, rules.clone());
        assert_eq!(man_update, vec![97, 75, 47, 29, 13]);
    }
}
