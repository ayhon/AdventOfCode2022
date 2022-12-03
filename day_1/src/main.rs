type ProblemData = Vec<u32>;
fn parse_input(input: Vec<String>) -> ProblemData {
    let mut res = Vec::new();
    let mut curr = 0;
    for line in input {
        if line.is_empty() {
            res.push(curr);
            curr = 0;
        } else {
            curr += line.parse::<u32>().expect("All inputs must be numbers or empty lines");
        }
    }
    return res;
}

fn part_one(input: &ProblemData) -> u32 {
    let mut max = 0;
    for calories in input {
        max = max.max(*calories);
    }
    return max;
}

const TOP_NUM: usize = 3;
use std::cmp::Reverse;
fn part_two(input: &ProblemData) -> u32 {
    let mut pq = std::collections::BinaryHeap::new();
    for num in input {
        pq.push(Reverse(num));
        if pq.len() > TOP_NUM {
            pq.pop();
        }
    }
    pq.iter().fold(0,|a,b|a+b.0)
}

fn main() {
    println!("Hello, world!");
    let stdin = std::io::stdin();
    let input = parse_input(stdin.lines().map(|x|x.unwrap()).collect());//::<Vec<String>>();
    let res_1 = part_one(&input);
    let res_2 = part_two(&input);
    assert!(res_1 <= res_2, "How can one elf carry more than the three elves which most carry?");
    println!("The elf with maximum calories carries {} calories", res_1);
    println!("The three elves which carry the most calories carry {} calories", res_2);
}
