use std::{fs, collections::HashSet};

use pathfinding::prelude::dijkstra;
use simple_matrix::Matrix;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

pub fn day12() {
    let mat: Matrix<i32> = Matrix::new(40, 95);
    let mut mat2 = add_map(&mat);    
    let start = find_value(&mat2, -14);
    let end = find_value(&mat2, -28);
    print!("Start {:?} end {:?} \n", start, end);
    //Set start and end positions to 0 level
    mat2.set(start.0 as usize, start.1 as usize, 0);

    let result = dijkstra(&start,|p| neighbours(p, &mat2), |p| *p == end).unwrap();
    let path = result.0.as_ref();
    print_matrix(&mat2,  path);
    print!("Total cost (part 1 answer) {:?}\n", result.1);
    
    //Part 2
    // Starting position can be any with value 0
    // Let's brute force
    // Find all positions with value 0
    let mut start_positions: Vec<Pos> = vec![];
    for i in 0..mat2.rows() - 1 {
        for j in 0..mat2.cols() - 1 {
            if mat2.get(i, j).unwrap().clone() == 0 {
                start_positions.push(Pos(i as i32, j as i32));
            }
        }
    }
    // Iterate over start_positions
    let costs = start_positions.iter().map(|start| {
        let result = dijkstra(start,|p| neighbours(p, &mat2), |p| *p == end);
        if let Some((path, length)) = result.as_ref() {
            return *length;
        } else {
            return 1000;
        }
        
    });
    print!("Min cost (part 2 answer) {:?}\n", costs.min().unwrap());


}

pub fn add_map(mat: &Matrix<i32>) -> Matrix<i32> {
    let mut mat_x = mat.clone();
    let message =
        fs::read_to_string("./input-day-12.txt").expect("Should have been able to read the file");
    let lines = message.lines();
    let mut li = 0;
    for line in lines {
        let chars = line.chars();
        let mut ci = 0;
        for char in chars {
            let value: i32 = char as i32;
            mat_x.set(li, ci, value - 97);
            ci += 1;
        }
        li += 1;
    }
    return mat_x;
}

fn print_matrix(mat: &Matrix<i32>, path: &Vec<Pos>) {
    let path_set: HashSet<Pos> = path.clone().into_iter().collect();
    for i in 0..mat.rows() - 1 {
        for j in 0..mat.cols() - 1 {
            if path_set.contains(&Pos(i as i32, j as i32)) {
                print!("X");
            } else {
                print!("{}", mat.get(i, j).unwrap());
            }
        }
        print!("\n");
    }
}

fn find_value(mat: &Matrix<i32>, value: i32) -> Pos {
    for i in 0..mat.rows() - 1 {
        for j in 0..mat.cols() - 1 {
            if mat.get(i, j).unwrap().clone() == value {
                return Pos(i as i32, j as i32);
            }
        }
    }
    return Pos(0, 0);
}

// for dikjstra implementation of pathfinding
fn neighbours(pos: &Pos, mat: &Matrix<i32>) -> Vec<(Pos, usize)> {
    let Pos(x, y) = pos;
    let height = get_height(pos, mat);
    let mut neighbours: Vec<(Pos, usize)> = vec![];

    let offsets = [(0, -1), (-1, 0), (1, 0), (0, 1)];
    for (dx, dy) in offsets.iter() {
        let x = x + dx;
        let y = y + dy;
        if x < 0 || y < 0 || x >= 40 || y >= 95 {
            continue;
        }
        let pos = Pos(x,y);
        let cost = get_cost(&pos, mat, height);
        if cost < 2 {
            neighbours.push((pos, 1));
        }
    }

    return neighbours;
}

fn get_cost(pos: &Pos, mat: &Matrix<i32>, original_height: i32) -> usize { 
    let height = get_height(pos, mat);
    if (height - original_height) > 1 {
        return 1000;
    }
    return 1;
}

fn get_height(pos: &Pos, mat: &Matrix<i32>) -> i32 {
    let x = pos.0 as usize;
    let y = pos.1 as usize;
    return  mat.get(x, y).unwrap_or(&100).clone();
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_height() {
        let mut mat: Matrix<i32> = Matrix::new(3, 3);
        mat.set(0, 0, 1);
        mat.set(1, 1, 2);
        let pos1 = Pos(0,0);
        let pos2 = Pos(1,1);
        assert_eq!(get_height(&pos1, &mat), 1);
        assert_eq!(get_height(&pos2, &mat), 2);
    }

    #[test]
    fn test_map() {
        let mat: Matrix<i32> = Matrix::new(40, 95);
        let mat2 = add_map(&mat);
        let pos = Pos(21,71);
        let n = neighbours(&pos, &mat2);
        assert_eq!(mat2.get(21, 71).unwrap(), &23);
        print!("Neighbours: {:?}\n", n);
    }
}