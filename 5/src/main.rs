use std::{collections::HashMap, str::FromStr, num::ParseIntError, cell::RefCell, borrow::BorrowMut};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct StackIdx(i32);

#[derive(Clone, Eq, PartialEq, Debug)]
struct Container(char);

#[derive(Clone, Debug)]
struct Procedure {
    amount: u32,
    from: StackIdx,
    to: StackIdx,
}

#[derive(Debug, Clone)]
enum ProcedureParseError {
    StringSplitError,
    IntegerParsingError(ParseIntError),
}

impl From<ParseIntError> for ProcedureParseError {
    fn from(value: ParseIntError) -> Self {
        ProcedureParseError::IntegerParsingError(value)
    }
}

impl FromStr for Procedure {
    type Err = ProcedureParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Format: move [amount] from [from] to [to]
        let mut it = s.split_whitespace();
        let amount = it.nth(1).ok_or(ProcedureParseError::StringSplitError)?.parse::<u32>()?;
        let from = it.nth(1).ok_or(ProcedureParseError::StringSplitError)?.parse::<i32>()?;
        let to = it.nth(1).ok_or(ProcedureParseError::StringSplitError)?.parse::<i32>()?;

        Ok(Procedure {
            amount,
            from: StackIdx(from),
            to: StackIdx(to),
        })
    }
}

fn split_input_types(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut input = input.split_terminator("\n\n");
    let starting_stacks_input = input.next().unwrap();
    let procedures = input.next().unwrap();
    (starting_stacks_input.lines().collect(), procedures.lines().collect())
}

fn initialize_stacks(mut starting_stacks_input: Vec<&str>) -> (HashMap<StackIdx, RefCell<Vec<Container>>>, Vec<StackIdx>) {

    let last_line = *starting_stacks_input.last().unwrap();
    let max_entries = starting_stacks_input.len();

    let stack_idxs: Vec<StackIdx> = last_line.split_whitespace().map(|idx| StackIdx(idx.parse::<i32>().unwrap())).collect();
    let stacks: HashMap<StackIdx, RefCell<Vec<Container>>> = stack_idxs.iter().map(
        |idx| (idx.clone(), RefCell::new(Vec::with_capacity(max_entries)))
    ).collect();

    starting_stacks_input.pop();
    starting_stacks_input.reverse();

    for stack_line in starting_stacks_input {
        for (stack_idx, possible_container) in split_at_containers(stack_line, &stack_idxs) {
            if let Some(container) = possible_container {
                stacks.get(&stack_idx).unwrap().borrow_mut().push(container);
            }
        }
    }

    (stacks, stack_idxs)

}

fn parse_and_execute_procedures(procedures_input: Vec<&str>, stacks: &HashMap<StackIdx, RefCell<Vec<Container>>>) {

    for procedure_str in procedures_input.iter() {
        let procedure = procedure_str.parse::<Procedure>().unwrap();

        let from = &procedure.from;
        let to = &procedure.to;

        if from != to {

            let mut from = stacks.get(from).unwrap().borrow_mut();
            let mut to = stacks.get(to).unwrap().borrow_mut();

            let from_length = from.len();
            if from_length < procedure.amount as usize {
                panic!(
                    "Not enough containers in stack {:?} to move {} containers.
                     Instruction was: {procedure_str}.
                     Stack contents: {:?}",
                    procedure.from, procedure.amount, stacks
                );
            }
    
            let mut to_insert = from.split_off(from_length - procedure.amount as usize);
            to_insert.reverse(); // Part 1
            to.append(&mut to_insert);
        }

    }

}

fn get_top_containers(stacks: HashMap<StackIdx, RefCell<Vec<Container>>>, stack_idxs: Vec<StackIdx>) -> String {

    let mut top_containers = String::new();

    for stack_idx in stack_idxs {
        let stack = stacks.get(&stack_idx).unwrap().borrow();
        top_containers.push_str(stack.last().unwrap().0.to_string().as_str());
    }

    top_containers

}

fn main() {
    let input = include_str!("../input.txt");
    let (starting_stacks_input, procedures_input) = split_input_types(input);
    let (stacks, stack_idxs) = initialize_stacks(starting_stacks_input);

    parse_and_execute_procedures(procedures_input, &stacks);

    let solution = get_top_containers(stacks, stack_idxs);

    println!("Top containers: {solution}");

}

fn split_at_containers(line: &str, stack_idxs: &[StackIdx]) -> Vec<(StackIdx, Option<Container>)> {

    const SPACING: usize = "[A]".len() + 1;

    let mut containers_to_insert: Vec<(StackIdx, Option<Container>)> = Vec::new();

    for idx in (0..line.len()).step_by(SPACING) {
        let stack_idx = stack_idxs[idx / SPACING].clone();
        let mut upper_limit = idx + SPACING;
        if upper_limit > line.len() {
            upper_limit = line.len();
        }

        containers_to_insert.push((stack_idx, clean_container(&line[idx..upper_limit])));
    }

    containers_to_insert

}

fn clean_container(container: &str) -> Option<Container> {
    let mut it = container.chars();
    match it.nth(1) {
        Some(' ') => None,
        Some(c) => Some(Container(c)),
        None => None,
    }
}