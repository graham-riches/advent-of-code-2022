extern crate utilities;

fn main() {
    let grid: Vec<Vec<i32>> = utilities::lines_from_file("input.txt").unwrap()
     .into_iter()
     .map(|s| s.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>())
     .collect();
    
    let visible = count_visible_trees(&grid);
    println!("Total visible: {}", visible);

    let best_view = get_max_scenic_score(&grid);
    println!("Best view: {}", best_view);
}

// Part one
fn count_visible_trees(grid: &Vec<Vec<i32>>) -> usize {
    let mut s: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
    
    // Helper to build out the set doing forward and reverse scans at the same time
    fn scan_visible(g: &Vec<Vec<i32>>, map: &mut Vec<Vec<bool>>, is_row: bool) {        
        for i in 0..g.len() {
            // Scan forwards
            let mut max: i32 = -1;
            for j in 0..g.len() {
                let r = if is_row { i } else { j };
                let c = if is_row { j } else { i };
                if g[r][c] > max {
                    max = g[r][c];
                    map[r][c] = true;
                }
            }
            
            // And reverse
            max = -1;
            for j in (0..g.len()).rev() {
                let r = if is_row { i } else { j };
                let c = if is_row { j } else { i };
                if g[r][c] > max {
                    max = g[r][c];
                    map[r][c] = true;
                }
            }
        }
    }
    // Scan both ways
    scan_visible(&grid, &mut s, true);
    scan_visible(&grid, &mut s, false);

    s.iter()
     .map(|x| x.iter().filter(|y| **y).count())
     .fold(0, |sum, count| sum + count)
}

// Part two scenic score scan
fn get_max_scenic_score(grid: &Vec<Vec<i32>>) -> i32 {    
    let mut max_score: i32 = -1;
    let rows = grid.len() - 1;
    let cols = grid[0].len() - 1;

    for i in 1..rows {
        for j in 1..cols {                                            
            let vr = view_scan(&grid, i, j, j + 1..cols, true);
            let vl = view_scan(&grid, i, j, (1..=j - 1).rev(), true);
            let vu = view_scan(&grid, i, j, (1..=i - 1).rev(), false);
            let vd = view_scan(&grid, i, j, i + 1..rows, false);
            let view_score = vr * vl * vu * vd;
            if view_score > max_score {
                max_score = view_score;
            }
        }
    }
    max_score
}

// Helper to do a view scan for a row
fn view_scan<I>(g: &Vec<Vec<i32>>, row: usize, column: usize, r: I, is_row: bool) -> i32 
where
    I: Iterator<Item = usize>    
{
    let mut count = 1;
    for ind in r {
        let t = if is_row { g[row][ind] } else { g[ind][column] };
        if t >= g[row][column] {
            break;
        }
        count += 1;
    }
    count
}
