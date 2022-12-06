fn find_k_different(input: &String, k: usize) -> Option<usize> {
    use std::collections::HashSet as Set;
    let mut seen = Set::<char>::new();
    let mut j = 0;
    let chars = input.chars().collect::<Vec<char>>();
    for (i,c) in chars.iter().enumerate() {
        while seen.contains(&c) {
            seen.remove(&chars[j]);
            j += 1;
        }
        seen.insert(*c);
        if seen.len() == k {
            return Some(i+1);
        }
    }
    return None;
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Must be given a line of input");
    println!("First marker character for start-of-packet after {}", find_k_different(&input,4).unwrap());
    println!("First marker character for message after {}", find_k_different(&input,14).unwrap());
}
