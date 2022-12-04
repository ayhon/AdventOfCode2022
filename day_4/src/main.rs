use std::ops::RangeInclusive;

type Sections = RangeInclusive<u32>;

fn is_subset_of(a: &Sections, b: &Sections) -> bool {
    a.start() <= b.start() && b.end() <= a.end()
}
fn sections_intersect(a: &Sections, b: &Sections) -> bool {
    b.contains(a.start()) != b.contains(a.end()) ||  is_subset_of(a,b) || is_subset_of(b,a)
}

fn read_sections(s: &str) -> Sections {
    let mut iter = s.split('-');
    let low = iter.next().expect("Couldn't find low part of range").parse().expect("Low part of range must be a number");
    let high = iter.next().expect("Couldn't find high part of range").parse().expect("High part of range must be a number");
    low..=high
}
mod part_one {
    use super::*;

    pub fn parse_input(input: &Vec<String>) -> Vec<(Sections,Sections)> {
        input.iter()
            .map(|l|{
                let mut iter = l.split(',');
                let lhs = iter.next().expect("Couldn't find information on the first range");
                let rhs = iter.next().expect("Couldn't find information on the second range");
                (read_sections(lhs),read_sections(rhs))
            })
            .collect()
    }

    fn solve(sections: Vec<(Sections,Sections)>) -> usize {
        sections.iter()
            .map(|(lhs,rhs)| (is_subset_of(lhs,rhs) || is_subset_of(rhs,lhs)) as usize )
            .sum()
    }

    pub fn main(input: &Vec<String>) -> usize {
        let sections = parse_input(input);
        solve(sections)
    }
}
mod part_two {
    use super::*;
    fn solve(sections: Vec<(Sections,Sections)>) -> usize {
        sections.iter()
            .map(|(lhs,rhs)| (sections_intersect(lhs,rhs) as usize))
            .sum()
    }
    pub fn main(input: &Vec<String>) -> usize {
        let sections = part_one::parse_input(input);
        solve(sections)
    }
}

fn main() {
    println!("Hello, world!");
    let input = std::io::stdin().lines().map(|x|x.unwrap()).collect();
    let res_one = part_one::main(&input);
    println!("One range fully contains the other in {} assigments", res_one);
    let res_two = part_two::main(&input);
    println!("Ranges intersect in {} assigments", res_two);

}
