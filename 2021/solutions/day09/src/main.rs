use utils::read_input;

use std::collections::{HashMap, HashSet};

// i believe this is what they call 1-step
fn calc_risk(heightmap: &HashMap<usize, Vec<u16>>) -> (u16, Vec<(usize, usize)>) {
    let mut risk_level = 0;
    let mut low_points: Vec<(usize, usize)> = vec![];

    let heightmap_size = heightmap.len();
    for i in 0..heightmap_size {
        let line_len = heightmap[&i].len();
        for j in 0..line_len {
            let val: u16 = heightmap.get(&i).unwrap()[j];

            match (i, j) {
                (0, 0) => {
                    let ip1 = heightmap.get(&(i + 1)).unwrap()[j];
                    let jp1 = heightmap.get(&i).unwrap()[j + 1];

                    if val < ip1 && val < jp1 {
                        risk_level += val + 1;
                        low_points.push((i, j));
                    }
                }
                (0, j) if j == line_len - 1 => {
                    let ip1 = heightmap.get(&(i + 1)).unwrap()[j];
                    let jm1 = heightmap.get(&i).unwrap()[j - 1];

                    if val < ip1 && val < jm1 {
                        risk_level += val + 1;
                        low_points.push((i, j));
                    }
                }
                (0, _) => {
                    let ip1 = heightmap.get(&(i + 1)).unwrap()[j];
                    let jp1 = heightmap.get(&i).unwrap()[j + 1];
                    let jm1 = heightmap.get(&i).unwrap()[j - 1];

                    if val < ip1 && val < jp1 && val < jm1 {
                        risk_level += val + 1;
                        low_points.push((i, j));
                    }
                }
                (i, j) if i == heightmap_size - 1 && j == 0 => {
                    let im1 = heightmap.get(&(i - 1)).unwrap()[j];
                    let jp1 = heightmap.get(&i).unwrap()[j + 1];

                    if val < im1 && val < jp1 {
                        risk_level += val + 1;
                        low_points.push((i, j));
                    }
                }

                (i, j) if i == heightmap_size - 1 && j == line_len - 1 => {
                    let im1 = heightmap.get(&(i - 1)).unwrap()[j];
                    let jm1 = heightmap.get(&i).unwrap()[j - 1];

                    if val < im1 && val < jm1 {
                        risk_level += val + 1;
                        low_points.push((i, j));
                    }
                }
                (i, _) if i == heightmap_size - 1 => {
                    let im1 = heightmap.get(&(i - 1)).unwrap()[j];
                    let jp1 = heightmap.get(&i).unwrap()[j + 1];
                    let jm1 = heightmap.get(&i).unwrap()[j - 1];

                    if val < im1 && val < jp1 && val < jm1 {
                        risk_level += val + 1;
                        low_points.push((i, j));
                    }
                }
                (_, j) if j == 0 => {
                    let ip1 = heightmap.get(&(i + 1)).unwrap()[j];
                    let im1 = heightmap.get(&(i - 1)).unwrap()[j];
                    let jp1 = heightmap.get(&i).unwrap()[j + 1];

                    if val < ip1 && val < im1 && val < jp1 {
                        risk_level += val + 1;
                        low_points.push((i, j));
                    }
                }
                (_, j) if j == line_len - 1 => {
                    let ip1 = heightmap.get(&(i + 1)).unwrap()[j];
                    let im1 = heightmap.get(&(i - 1)).unwrap()[j];
                    let jm1 = heightmap.get(&i).unwrap()[j - 1];

                    if val < ip1 && val < im1 && val < jm1 {
                        risk_level += val + 1;
                        low_points.push((i, j));
                    }
                }
                _ => {
                    let ip1 = heightmap.get(&(i + 1)).unwrap()[j];
                    let im1 = heightmap.get(&(i - 1)).unwrap()[j];
                    let jp1 = heightmap.get(&i).unwrap()[j + 1];
                    let jm1 = heightmap.get(&i).unwrap()[j - 1];

                    if val < ip1 && val < im1 && val < jp1 && val < jm1 {
                        risk_level += val + 1;
                        low_points.push((i, j));
                    }
                }
            }
        }
    }

    return (risk_level, low_points);
}

fn calc_basin_size(low_points: Vec<(usize, usize)>, heightmap: &HashMap<usize, Vec<u16>>) -> usize {
    let mut sizes: Vec<usize> = vec![];
    for (i, j) in low_points {
        let mut visited = HashSet::new();
        dfs(i, j, heightmap, &mut visited);

        sizes.push(visited.len());
    }

    // sort in descending order
    // https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust
    sizes.sort_by(|a, b| b.cmp(a));
    sizes[0] * sizes[1] * sizes[2]
}

// prolly should've found part1 using dfs, but idc
fn dfs(
    i: usize,
    j: usize,
    heightmap: &HashMap<usize, Vec<u16>>,
    visited: &mut HashSet<(usize, usize)>,
) {
    visited.insert((i, j));

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for (di, dj) in directions.iter() {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;

        if ni >= 0 && nj >= 0 {
            let ni = ni as usize;
            let nj = nj as usize;
            if let Some(row) = heightmap.get(&ni) {
                if nj < row.len() && !visited.contains(&(ni, nj)) && row[nj] != 9 {
                    dfs(ni, nj, heightmap, visited);
                }
            }
        }
    }
}

fn main() {
    let input = read_input("./inputs/9");
    let lines = input.lines();
    let mut heightmap: HashMap<usize, Vec<u16>> = HashMap::new();

    lines.enumerate().for_each(|(index, line)| {
        heightmap.insert(
            index,
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u16)
                .collect(),
        );
    });

    let (risk_score, low_points) = calc_risk(&heightmap);

    println!("09 part1: {}", risk_score);
    println!("09 part2: {}", calc_basin_size(low_points, &heightmap));
}
