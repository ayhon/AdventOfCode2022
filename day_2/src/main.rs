#[derive(Clone)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}
impl Shape {
    fn from(a: &char) -> Option<Shape> {
        match a {
            'A' | 'X' => Some(Rock),
            'B' | 'Y' => Some(Paper),
            'C' | 'Z' => Some(Scissors),
            _ => None,
        }
    }
}
impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Rock => write!(f,"rock"),
            Paper => write!(f,"paper"),
            Scissors => write!(f,"scissors"),
        }
    }
}
use Shape::*;

pub struct Game(Shape,Shape);
impl Game {
    fn from(a: &str) -> Option<Game> {
        let mut chars = a.chars();
        let first = chars.next()?;
        chars.next(); // Skip space
        let second = chars.next()?;
        Some(Game(Shape::from(&first)?,
            Shape::from(&second)?))
    }
}
impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,"{} vs {}", self.0, self.1)
    }
}

pub enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6
}
impl Outcome {
    fn from(c: &char) -> Option<Outcome> {
        match c {
            'X' => Some(Lose),
            'Y' => Some(Draw),
            'Z' => Some(Win),
            _ => None
        }
    }
    fn given_shape(&self,shape: &Shape) -> Shape {
        match (self,shape) {
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            (Draw, _) => Shape::clone(shape),
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
        }
    }
}
use Outcome::*;

mod part_one {
    use crate::*;
    pub fn score(game: &Game) -> usize {
        let res = score_game(&game) + score_play(&game.1);
        eprintln!("Scored {} points",res);
        res
    }
    fn score_game(game: &Game) -> usize {
        eprint!("Game {} ",game);
        match game {
            Game(Rock,Paper) | Game(Paper,Scissors) | Game(Scissors,Rock) => { 
                eprintln!("won.");
                Win as usize
            },
            Game(Rock, Rock) | Game(Paper, Paper)   | Game(Scissors, Scissors) => {
                eprintln!("draw.");
                Draw as usize
            },
            Game(Paper, Rock)| Game(Rock, Scissors) | Game(Scissors, Paper) => {
                eprintln!("lost.");
                Lose as usize
            },
        }
    }
    fn score_play(play: &Shape) -> usize {
        Shape::clone(play) as usize
    }

    fn parse_input(input: &Vec<String>) -> Vec<Game> {
        input.iter()
            .map(|l|Game::from(&l).expect("Wrong format for game"))
            .collect()
    }

    pub fn solve(input: &Vec<String>) -> usize {
        parse_input(input).iter().map(score).sum()
    }
}



mod part_two {
    use crate::*;
    fn parse_input(input: &Vec<String>) -> Vec<Game> {
        input.iter()
            .map(|l| {
                let mut chars = l.chars();
                let other_shape = Shape::from(&chars.next().unwrap()).expect("Incorrect encoding for shape");
                chars.next();
                let my_shape = Outcome::from(&chars.next().unwrap()).expect("Incorrect encoding for outcome")
                                .given_shape(&other_shape);
                Game(other_shape,my_shape)
            }).collect()
    }

    pub fn solve(input: &Vec<String>) -> usize {
        parse_input(input).iter().map(part_one::score).sum()
    }
}

fn main() {
    let input = std::io::stdin().lines().map(|x|x.unwrap()).collect();
    let res_1 = part_one::solve(&input);
    println!("The result of playing the game with the deduced given strategy is {}",res_1);
    let res_2 = part_two::solve(&input);
    println!("The result of playing the game with the actual given strategy is {}",res_2);
}
