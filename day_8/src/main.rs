type HeightGrid = Vec<Vec<i8>>;
const DIRS: [(isize,isize);4] = [(0,1),(1,0),(0,-1),(-1,0)];

fn get_dimensions(grid: &HeightGrid) -> (usize,usize) {
    (grid.len(), grid.first().expect("Grid cannot be empty").len())
}

fn compute_visibility_matrix(grid: &HeightGrid) -> Vec<Vec<bool>> {
    let (n,m) = get_dimensions(grid);
    let mut res = vec![vec![false; m]; n];
    for dir in DIRS {
        compute_visibility_matrix_along_direction(grid,dir,&mut res);
    }
    res
}

/*

 → ( 0,  1)
 ↑ ( 1,  0)
 ← ( 0, -1)
 ↓ (-1,  0)

*/
fn compute_visibility_matrix_along_direction(grid: &HeightGrid, dir: (isize,isize), res: &mut Vec<Vec<bool>>) {
    let (n,m) = get_dimensions(grid);
    // eprintln!("Direction {:?}",dir);
    for i in 0..n {
        let mut curr_max = -1;
        for j in 0..m {
            let (x,y) = if dir.0 == 0 {
                if dir.1 > 0 {
                    (j,i)
                } else {
                    (m-1 - j,n-1 - i)
                }
            } else if dir.0 > 0 {
                (i,j)
            } else {
                (i,m-1 - j)
            };
            let elem = grid[x][y];
            if curr_max < elem {
                curr_max = elem;
                res[x][y] = true;
                // eprint!("V");
            } else {
                // eprint!(" ");
            }
            // eprint!("{},{} ",x,y);
        }
        // eprintln!()
    }
}

fn viewing_distance(grid: &HeightGrid, dir: (isize,isize), i: usize, j: usize) -> i32 {
    let (n,m) = get_dimensions(grid);
    let mut viewing_distance = 0;
    let top_height = grid[i][j];
    let mut x = (i as isize) + dir.0;
    let mut y = (j as isize) + dir.1;
    while 0 <= x && x < (n as isize) && 0 <= y && y < (m as isize) {
        viewing_distance+=1;
        if grid[x as usize][y as usize] >= top_height {
            break;
        }
        y+=dir.1;
        x+=dir.0;
    }
    viewing_distance
}

fn scenic_score(grid: &HeightGrid, i: usize, j: usize) -> i32 {
    let mut res = 1;
    // eprintln!("Looking at {}, {}",i,j);
    for dir in DIRS {
        let vd = viewing_distance(grid,dir,i,j);
        // eprintln!("  Dir {},{} got {}",dir.0,dir.1,vd);
        res *= vd;
    }
    res
}

fn main() {
    println!("Hello, world!");
    let grid: HeightGrid = std::io::stdin()
        .lines()
        .map( |x|
            x.unwrap()
             .chars()
             .map(|c|c.to_digit(10).expect("Input must be a digit") as i8)
             .collect()
        )
        .collect();

    let vis_mat = compute_visibility_matrix(&grid);
    let mut res_one = 0;
    for row in vis_mat { for elem in row { if elem { res_one += 1 } } }
    println!("Res one: {}",res_one);

    // dbg!(&grid);
    let mut res_two = 0;
    let (n,m) = get_dimensions(&grid);
    for i in 0..n {
        for j in 0..m {
            res_two = res_two.max(scenic_score(&grid,i,j));
        }
    }
    println!("Res two: {}",res_two);
}
