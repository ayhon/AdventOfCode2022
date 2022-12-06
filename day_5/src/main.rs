#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

#[derive(Debug)]
pub struct Problem {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>
}

impl Problem {
    fn new(n: usize) -> Problem {
        Problem { stacks: vec![Vec::new();n], moves: Vec::new() }
    }
    fn perform(stacks: &mut Vec<Vec<char>>, m: &Move) {
        let Move{from:f,to:t,amount} = *m;
        if t == f { return }
        let (from, to) = {
            if t < f {
                let (contains_to, contains_from) = stacks.split_at_mut(f);
                (&contains_from[0], &mut contains_to[t])
            } else {
                let (contains_from, contains_to) = stacks.split_at_mut(t);
                (&contains_from[f], &mut contains_to[0])
            }
        };
        to.extend(from[from.len() - amount ..].iter().rev());
        let from = &mut stacks[f];
        from.truncate(from.len() - amount);
        // dbg!(&stacks);
    }
    fn perform_part_two(stacks: &mut Vec<Vec<char>>, m: &Move) {
        let Move{from:f,to:t,amount} = *m;
        if t == f { return }
        let (from, to) = {
            if t < f {
                let (contains_to, contains_from) = stacks.split_at_mut(f);
                (&contains_from[0], &mut contains_to[t])
            } else {
                let (contains_from, contains_to) = stacks.split_at_mut(t);
                (&contains_from[f], &mut contains_to[0])
            }
        };
        to.extend(from[from.len() - amount ..].iter());
        let from = &mut stacks[f];
        from.truncate(from.len() - amount);
        // dbg!(&stacks);
    }
}

mod part_one {
    use super::*;
    fn parse_stack_line(p: &mut Problem, s: &str) {
        // TODO: Change to work always
        let mut chars = s.chars();
        let mut i = 0;
        while let Some(_) = chars.next() {
            let char = chars.next().expect("Unrecognized format for crate element");
            if char != ' ' { p.stacks[i].push(char); }
            chars.next();
            chars.next();
            i += 1;
        }
    }
    pub fn parse_input(input: &Vec<String>) -> Problem {
        let mut iter = input.iter();

        let first_line = iter.next().expect("Input can't be empty");
        let mut problem = Problem::new( (first_line.len()+1) / 4);
        parse_stack_line(&mut problem, first_line);
        while let Some(line) = iter.next() {
            if !line.trim_start().starts_with("[") { break }
            parse_stack_line(&mut problem, line);
        }
        for stack in &mut problem.stacks[..] {
            stack.reverse();
        }
        iter.next();

        while let Some(line) = iter.next() {
            match line.split(' ').collect::<Vec<&str>>()[..] {
                [_, amount, _, from, _, to] => {
                    problem.moves.push(Move{
                        from: from.parse::<usize>().expect("From must be a number") - 1,
                        to: to.parse::<usize>().expect("To must be a number") - 1,
                        amount: amount.parse().expect("Amount must be a number"),
                    });
                },
                _ => panic!("Invalid format for move description"),
            }
        }
        // dbg!(&problem);

        problem
    }

    pub fn solve(problem: &mut Problem) -> String {
        for m in &problem.moves[..] {
            Problem::perform(&mut problem.stacks, &m);
        }
        problem.stacks.iter().map(|stack|stack.last().unwrap().to_string()).collect::<Vec<String>>().join("")
    }

    pub fn main(input: Vec<String>) -> String {
        solve(&mut parse_input(&input))
    }
}

mod part_two {
    use crate::*;
    pub fn main(input: Vec<String>) -> String {
        let mut problem = part_one::parse_input(&input);
        for m in &problem.moves[..] {
            Problem::perform_part_two(&mut problem.stacks, &m);
        }
        problem.stacks.iter().map(|stack|stack.last().unwrap().to_string()).collect::<Vec<String>>().join("")
    }
}

fn main() {
    println!("Hello, world!");
    let input: Vec<String> = std::io::stdin().lines().map(|x|x.unwrap()).collect();
    let res_one = part_one::main(input.clone());
    println!("The top containers after the operations are: {}", res_one);
    let res_two = part_two::main(input);
    println!("The top containers after the operations are: {}", res_two);
}
