use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

struct Tile {
    id: u64,
    edges: Vec<u16>,
}

pub fn solve(part: u64, filename: String) {
    let input_data = fs::read_to_string(filename).expect("Could not open file for reading");
    let lines = input_data.lines().collect::<Vec<&str>>();
    let re = Regex::new(r"Tile (?P<tile>[0-9]*):").unwrap();
    let dimension = lines[1].chars().collect::<Vec<char>>().len();
    let mut tiles: Vec<Tile> = Vec::new();

    let mut i = 0;
    while i < lines.len() {
        let tile_no = re.replace_all(lines[i], "$tile").parse::<u64>().unwrap();
        let top = lines[i + 1]
            .chars()
            .map(|x| match x {
                '#' => 1,
                '.' => 0,
                _ => 0,
            })
            .fold(0, |x, y| x * 2 + y);
        let right = (1..=dimension)
            .map(
                |x| match lines[i + x].chars().collect::<Vec<char>>()[dimension - 1] {
                    '#' => 1,
                    '.' => 0,
                    _ => 0,
                },
            )
            .fold(0, |x, y| x * 2 + y);
        let bottom = lines[i + dimension]
            .chars()
            .map(|x| match x {
                '#' => 1,
                '.' => 0,
                _ => 0,
            })
            .fold(0, |x, y| x * 2 + y);
        let left = (1..=dimension)
            .map(|x| match lines[i + x].chars().collect::<Vec<char>>()[0] {
                '#' => 1,
                '.' => 0,
                _ => 0,
            })
            .fold(0, |x, y| x * 2 + y);

        tiles.push(Tile {
            id: tile_no,
            edges: vec![top, right, bottom, left],
        });
        i += dimension + 2;
    }

    let now = Instant::now();
    match part {
        1 => solve_part1(dimension, tiles),
        2 => solve_part2(dimension, tiles),
        _ => {}
    }
    println!("Time: {:4} ms", now.elapsed().as_micros());
}

fn reverse(num: u16, dimension: usize) -> u16 {
    (0..dimension)
        .map(|x| if num & 1 << x == 0 { 0 } else { 1 })
        .fold(0, |x, y| 2 * x + y)
}

fn solve_part1(dimension: usize, tiles: Vec<Tile>) {
    // Jigsaw puzzle
    let mut edge_map: HashMap<u16, Vec<usize>> = HashMap::new();
    for (index, tile) in tiles.iter().enumerate() {
        for edge in &tile.edges {
            let edge_key;
            let reverse_edge = reverse(*edge, dimension);
            if *edge < reverse_edge {
                edge_key = *edge;
            } else {
                edge_key = reverse_edge;
            }
            if edge_map.contains_key(&edge_key) {
                edge_map.get_mut(&edge_key).unwrap().push(index);
            } else {
                edge_map.insert(edge_key, vec![index]);
            }
        }
    }

    let edge_tiles = edge_map
        .iter()
        .filter_map(|x| if x.1.len() == 1 { Some(x.1[0]) } else { None })
        .collect::<Vec<usize>>();
    let mut free_tiles: HashSet<usize> = HashSet::new();
    let mut corners: Vec<usize> = Vec::new();
    for val in edge_tiles {
        if free_tiles.contains(&val) {
            corners.push(val);
        } else {
            free_tiles.insert(val);
        }
    }

    println!("{}", corners.iter().map(|&x| tiles[x].id).product::<u64>());
}

/*
fn orient(
    tile: &Tile,
    left: Option<u16>,
    top: Option<u16>,
    outer_ring_edges: &HashSet<u16>,
    dimension: usize,
) -> Option<(bool, bool, u16, u16, u16)> {
    match left {
        Some(left_edge_val) => {
            match top {
                Some(top_edge_val) => {
                    // Both top edge and left edge are fixed

                    // Check the initial tile
                    let edge_vec = tile.edges.clone();
                    for i in 0..edge_vec.len() {
                        if edge_vec[i] == top_edge_val && edge_vec[(i + 3) % 4] == left_edge_val {
                            (false, false, i, edge_vec[i + 1], edge_vec[i + 2])
                        }
                    }

                    // Flip in X direction
                    let new_vec = vec![
                        reverse(edge_vec[0], dimension),
                        edge_vec[3],
                        reverse(edge_vec[2], dimension),
                        edge_vec[1],
                    ];
                    for i in 0..new_vec.len() {
                        if new_vec[i] == top_edge_val && new_vec[(i + 3) % 4] == left_edge_val {
                            (true, false, i, new_vec[i + 1], new_vec[i + 2])
                        }
                    }

                    // Flip in Y direction
                    let new_vec = vec![
                        edge_vec[2],
                        reverse(edge_vec[1], dimension),
                        edge_vec[0],
                        reverse(edge_vec[3], dimension),
                    ];
                    for i in 0..new_vec.len() {
                        if new_vec[i] == top_edge_val && new_vec[(i + 3) % 4] == left_edge_val {
                            (false, true, i, new_vec[i + 1], new_vec[i + 2])
                        }
                    }

                    // Flip in X and Y direction
                    let new_vec = vec![
                        reverse(edge_vec[2], dimension),
                        reverse(edge_vec[3], dimension),
                        reverse(edge_vec[0], dimension),
                        reverse(edge_vec[1], dimension),
                    ];
                    for i in 0..new_vec.len() {
                        if new_vec[i] == top_edge_val && new_vec[(i + 3) % 4] == left_edge_val {
                            (true, true, i, new_vec[i + 1], new_vec[i + 2])
                        }
                    }
                }
                None => {
                    // Top edge is an boundary edge

                    // Check the initial tile
                    let edge_vec = tile.edges.clone();
                    for i in 0..edge_vec.len() {
                        if outer_ring_edges.contains(edge_vec[i])
                            && edge_vec[(i + 3) % 4] == left_edge_val
                        {
                            (false, false, i, edge_vec[i + 1], edge_vec[i + 2])
                        }
                    }

                    // Flip in X direction
                    let new_vec = vec![
                        reverse(edge_vec[0], dimension),
                        edge_vec[3],
                        reverse(edge_vec[2], dimension),
                        edge_vec[1],
                    ];
                    for i in 0..new_vec.len() {
                        if outer_ring_edges.contains(new_vec[i])
                            && new_vec[(i + 3) % 4] == left_edge_val
                        {
                            (true, false, i, new_vec[i + 1], new_vec[i + 2])
                        }
                    }

                    // Flip in Y direction
                    let new_vec = vec![
                        edge_vec[2],
                        reverse(edge_vec[1], dimension),
                        edge_vec[0],
                        reverse(edge_vec[3], dimension),
                    ];
                    for i in 0..new_vec.len() {
                        if outer_ring_edges.contains(new_vec[i])
                            && new_vec[(i + 3) % 4] == left_edge_val
                        {
                            (false, true, i, new_vec[i + 1], new_vec[i + 2])
                        }
                    }

                    // Flip in X and Y direction
                    let new_vec = vec![
                        reverse(edge_vec[2], dimension),
                        reverse(edge_vec[3], dimension),
                        reverse(edge_vec[0], dimension),
                        reverse(edge_vec[1], dimension),
                    ];
                    for i in 0..new_vec.len() {
                        if outer_ring_edges.contains(new_vec[i])
                            && new_vec[(i + 3) % 4] == left_edge_val
                        {
                            (true, true, i, new_vec[i + 1], new_vec[i + 2])
                        }
                    }
                }
            }
        }
        None => {
            match top {
                Some(top_edge_val) => {
                    // Left edge is an boundary edge

                    // Check the initial tile
                    let edge_vec = tile.edges.clone();
                    for i in 0..edge_vec.len() {
                        if outer_ring_edges.contains(edge_vec[i])
                            && edge_vec[(i + 3) % 4] == left_edge_val
                        {
                            (false, false, i, edge_vec[i + 1], edge_vec[i + 2])
                        }
                    }

                    // Flip in X direction
                    let new_vec = vec![
                        reverse(edge_vec[0], dimension),
                        edge_vec[3],
                        reverse(edge_vec[2], dimension),
                        edge_vec[1],
                    ];
                    for i in 0..new_vec.len() {
                        if outer_ring_edges.contains(new_vec[i])
                            && new_vec[(i + 3) % 4] == left_edge_val
                        {
                            (true, false, i, new_vec[i + 1], new_vec[i + 2])
                        }
                    }

                    // Flip in Y direction
                    let new_vec = vec![
                        edge_vec[2],
                        reverse(edge_vec[1], dimension),
                        edge_vec[0],
                        reverse(edge_vec[3], dimension),
                    ];
                    for i in 0..new_vec.len() {
                        if outer_ring_edges.contains(new_vec[i])
                            && new_vec[(i + 3) % 4] == left_edge_val
                        {
                            (false, true, i, new_vec[i + 1], new_vec[i + 2])
                        }
                    }

                    // Flip in X and Y direction
                    let new_vec = vec![
                        reverse(edge_vec[2], dimension),
                        reverse(edge_vec[3], dimension),
                        reverse(edge_vec[0], dimension),
                        reverse(edge_vec[1], dimension),
                    ];
                    for i in 0..new_vec.len() {
                        if outer_ring_edges.contains(new_vec[i])
                            && new_vec[(i + 3) % 4] == left_edge_val
                        {
                            (true, true, i, new_vec[i + 1], new_vec[i + 2])
                        }
                    }
                }
                None => {
                    // Both edges are boundary edge
                    let edge_vec = tile.edges.clone();
                    for i in 0..edge_vec.len() {
                        if outer_ring_edges.contains(&edge_vec[i])
                            && outer_ring_edges.contains(&edge_vec[(i + 3) % 4])
                        {
                            (false, false, i as u16, edge_vec[i + 1], edge_vec[i + 2])
                        }
                    }
                }
            }
        }
    }
}
*/

fn solve_part2(dimension: usize, tiles: Vec<Tile>) {
    /*
    // Jigsaw puzzle
    //
    let mut edge_map: HashMap<u16, Vec<usize>> = HashMap::new();
    for (index, tile) in tiles.iter().enumerate() {
        for edge in &tile.edges {
            let edge_key;
            let reverse_edge = reverse(*edge, dimension);
            if *edge < reverse_edge {
                edge_key = *edge;
            } else {
                edge_key = reverse_edge;
            }
            if edge_map.contains_key(&edge_key) {
                edge_map.get_mut(&edge_key).unwrap().push(index);
            } else {
                edge_map.insert(edge_key, vec![index]);
            }
        }
    }

    let mut positions = vec![vec![0, tiles.len()]; tiles.len()];

    let edge_tiles = edge_map
        .iter()
        .filter_map(|x| {
            if x.1.len() == 1 {
                Some((*x.0, x.1[0]))
            } else {
                None
            }
        })
        .collect::<Vec<(u16, usize)>>();
    let mut outer_ring_tiles: HashSet<usize> = HashSet::new();
    let mut outer_ring_edges: HashSet<u16> = HashSet::new();
    let mut corners: Vec<usize> = Vec::new();
    for (edge_val, val) in edge_tiles {
        outer_ring_edges.insert(edge_val);
        if outer_ring_tiles.contains(&val) {
            corners.push(val);
        } else {
            outer_ring_tiles.insert(val);
        }
    }

    let mut x_flip = vec![false; tiles.len()];
    let mut y_flip = vec![false; tiles.len()];
    let mut rotation = vec![0; tiles.len()];

    println!("{}", corners.iter().map(|&x| tiles[x].id).product::<u64>());
    */
}
