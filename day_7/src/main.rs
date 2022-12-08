use std::collections::HashMap;
pub enum FSTree<'a> {
    File(usize),
    Directory{
        parent: usize,
        children: HashMap<&'a str,usize>,
    }
}
use FSTree::*;

const LIMIT: usize = 100000;
const FS_SIZE: usize   = 70000000;
const NEEDED_SIZE: usize = 30000000;

mod part_one {
    use super::*;
    pub fn parse_input(input: &Vec<String>) -> Vec<FSTree> {
        let mut nodes = vec![Directory{parent: 0,children: HashMap::new()}];
        let mut cwd = 0;
        for line in input {
            let len = nodes.len();
            if let Directory{parent, ref mut children} = nodes[cwd] {
                if line.starts_with("$ cd")  {
                    let dir_name = line.split(' ').skip(2).next().expect("Should provide a directory name to change to");
                    if dir_name == "/" {
                        cwd = 0;
                    } else if dir_name == ".." {
                        cwd = parent as usize;
                    } else  {
                        cwd = *children.get(dir_name).expect("Couldn't find the specified directory in the current working directory");
                    }
                } else if !line.starts_with("$") {
                    let mut comp = line.split(' ');
                    let size_or_dir = comp.next().expect("Expected size or directory mark before file_name");
                    let filename = comp.next().expect("Expected filename after size or directory mark");
                    if size_or_dir == "dir" {
                        children.insert(filename, len);
                        nodes.push(Directory { parent: cwd, children: HashMap::new() });
                    } else {
                        children.insert(filename, len);
                        nodes.push(File(size_or_dir.parse().expect("Wrong format for file size")));
                    }
                }
            } else {
                panic!("The current working directory should always be a directory");
            }
        }
        nodes
    }
    pub fn compute_size(nodes: &Vec<FSTree>, res: &mut Vec<usize>, i: usize) {
        let size = match &nodes[i] {
            Directory { children,.. } => {
                let mut size = 0;
                for child in children.values() {
                    compute_size(nodes,res,*child);
                    size += res[*child];
                }
                size
            }
            File(size) => *size
        };
        res[i] = size;
    }
    pub fn main(input: &Vec<String>) -> usize {
        let fs = parse_input(input);
        let mut sizes = vec![0;fs.len()];
        compute_size(&fs, &mut sizes, 0);
        let mut res = 0;
        for (i,file) in fs.iter().enumerate() {
            if let Directory {..} = file {
                if sizes[i] <= LIMIT {
                    res += sizes[i];
                }
            }
        }
        res
    }
}

mod part_two {
    use super::*;
    use part_one::*;
    pub fn main(input: &Vec<String>) -> usize {
        let fs = parse_input(input);
        let mut sizes = vec![0;fs.len()];
        compute_size(&fs, &mut sizes, 0);

        let mut res = sizes[0];
        let free_space = FS_SIZE - sizes[0];
        let need_to_free_up = NEEDED_SIZE - free_space;
        for (i,file) in fs.iter().enumerate() {
            if let Directory {..} = file {
                if sizes[i] >= need_to_free_up && sizes[i] < res {
                    res = sizes[i];
                }
            }
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
    let input = std::io::stdin().lines().map(|x|x.unwrap()).collect();
    let res_one = part_one::main(&input);
    println!("Res 1: {}", res_one);
    let res_two = part_two::main(&input);
    println!("Res 2: {}", res_two);
}
