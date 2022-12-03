use std::collections::BTreeSet as Set;

#[derive(Debug, Clone)]
pub struct Backpack(Set<char>,Set<char>);

mod part_one {
    use crate::*;
    pub fn parse_input(input: &Vec<String>) -> Vec<Backpack> {
        input.iter().map(|line|{
            let mut res = Backpack(Set::new(), Set::new());
            let (lhs, rhs) = line.split_at(line.len() / 2);
            for char in lhs.chars() {
                res.0.insert(char);
            }
            for char in rhs.chars() {
                res.1.insert(char);
            }
            res
        }).collect()
    }

    pub fn priority(c: char) -> usize {
        let ascii_code = c as usize;
        match c {
            'a'..='z' => ascii_code - ('a' as usize) + 1,
            'A'..='Z' => ascii_code - ('A' as usize) + 1 + 26,
            _ => panic!("Can only get the priority of lowercase and uppercase english letters"),
        }
    }

    fn solve(backpacks: Vec<Backpack>) -> usize {
        backpacks.iter()
            .map(|b|{
                let Backpack(lhs, rhs) = b;
                let mut common = lhs.intersection(rhs);
                let res;
                if let Some(elem) = common.next() {
                    res = priority(*elem);
                    eprintln!("The common element in the next backpack is {} with puntuation {}", elem, res);
                } else {
                    panic!("Backpacks should have an element in common in both compartments. Found {:?}",b);
                }
                assert!(common.next().is_none(), "Backpacks should only have one element in common in both compartments");
                res
            }).sum()
    }
    pub fn main(input: &Vec<String>) -> usize {
        solve(parse_input(input))
    }
}

mod part_two {
    use crate::*;
    struct Group(Backpack,Backpack,Backpack);
    fn parse_input(input: &Vec<String>) -> Vec<Group> {
        let backpacks = part_one::parse_input(input);
        let mut iter = backpacks.iter().cloned();
        (0..backpacks.len()/3)
            .map(move |_| Group(
                    iter.next().expect("This should have been guaranteed to never trigger"),
                    iter.next().expect("The number of backpacks should be a multiple of 3, but 1 remains"),
                    iter.next().expect("The number of backpacks should be a multiple of 3, but 2 remain")
                ))
            .collect()
    }
    fn solve(groups: &Vec<Group>) -> usize {
        groups.iter()
            .map(|g|{
                let Group(a,b,c) = g;
                let a_elems = a.0.union(&a.1).cloned().collect::<Set<char>>();
                let b_elems = b.0.union(&b.1).cloned().collect::<Set<char>>();
                let c_elems = c.0.union(&c.1).cloned().collect::<Set<char>>();
                let common = a_elems.intersection(&b_elems).cloned().collect::<Set<char>>().intersection(&c_elems).cloned().collect::<Set<char>>();
                let mut iter = common.iter();
                let badge = iter.next().expect("Members in a group should have a common badge");
                assert!(iter.next().is_none(), "Members in a group should only have one common element");
                part_one::priority(badge.clone())
            })
            .sum()
    }
    pub fn main(input: &Vec<String>) -> usize {
        solve(&parse_input(&input))
    }
}

fn main() {
    println!("Hello, world!");
    let input = std::io::stdin().lines().map(|x|x.unwrap()).filter(|x|!x.is_empty()).collect();
    let res_one = part_one::main(&input);
    println!("The total priority of the common objects is {}",res_one);
    let res_two = part_two::main(&input);
    println!("The total priority of the badges of each group is {}",res_two);
}
