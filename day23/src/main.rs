use std::{fs, env, collections::{HashSet, VecDeque}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    compute(&content);
}

fn compute(content: &String) {
    let lines = content.lines();
    let mut grid: Vec<Vec<char>> = vec!();
    lines.for_each(|line| grid.push(line.chars().collect()));
    let edge = (grid.len()-1, grid[0].len()-1);
    let start = (0,1);
    assert_eq!(grid[start.0][start.1], '.');
    grid[start.0][start.1] =  'v';
    println!("The longest route with slopes is: {}", longest_path(&grid, start, edge.0));
    // Building network
    let mut edge_graph: HashSet<Edge> = HashSet::new();
    {
        let mut queue: VecDeque<((usize,usize),(usize,usize))> = VecDeque::from([(start, (1,1))]);
        while let Some((departure, direction_coord)) = queue.pop_front() {
            explore_edge(&grid, departure, direction_coord, edge.0, &mut edge_graph, &mut queue);
        }
    }
    let mut visited = Vec::new();
    println!("The longest route without slopes is: {}", longest_path_graph(&edge_graph, (0,1), (edge.0, edge.1-1), &mut visited).expect("No path found"));
}

fn longest_path_graph(edge_graph: &HashSet<Edge>, current: (usize,usize), end: (usize,usize), visited: &mut Vec<(usize,usize)>) -> Option<usize> {
    if current == end { return Some(0); }
    let filtered: Vec<((usize,usize), usize)> = edge_graph.iter().filter_map(|edge| {
        if edge.node_1 == current && !visited.contains(&edge.node_2) {
            Some((edge.node_2, edge.weight))
        } else if edge.node_2 == current && !visited.contains(&edge.node_1) {
            Some((edge.node_1, edge.weight))
        } else {
            None
        }
    }).collect();
    visited.push(current);
    let max = filtered.iter().filter_map(|(edge,weight)|
        longest_path_graph(edge_graph, *edge, end, visited).map(|path_len| path_len + weight)
    ).max();
    visited.pop();
    max
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Edge {
    node_1: (usize,usize),
    node_2: (usize,usize),
    weight: usize,
}

fn explore_edge(grid: &Vec<Vec<char>>, departure_coord: (usize,usize), mut coord: (usize,usize), last_row: usize, edge_graph: &mut HashSet<Edge>, out: &mut VecDeque<((usize,usize),(usize,usize))>) {
    let mut next_out = Vec::new();
    let mut previous;

    get_next_no_slopes(grid, coord, &mut next_out);
    next_out.retain(|next| *next != departure_coord);
    let mut len = 1;
    while next_out.len() == 1 {
        len +=1;
        previous = coord;
        coord = next_out[0];
        if coord.0 == last_row {
            edge_graph.insert(Edge {node_1:departure_coord, node_2:coord, weight: len });
            return;
        }
        next_out.clear();
        get_next_no_slopes(grid, coord, &mut next_out);
        next_out.retain(|next| *next != previous);
    }
    if edge_graph.iter().any(|edge| edge.node_1 == coord && edge.node_2 == departure_coord ||
                                    edge.node_2 == coord && edge.node_1 == departure_coord) {
        return;
    }
    edge_graph.insert(Edge {node_1:departure_coord, node_2:coord, weight: len });
    next_out.iter().for_each(|next| out.push_back((coord, *next)));
}


fn longest_path(grid: &Vec<Vec<char>>, mut coord: (usize,usize), last_row: usize) -> usize {
    let mut out = Vec::new();
    let mut previous;
    get_next(grid, coord, &mut out);
    let mut len = 1;
    while out.len() == 1 {
        previous = coord;
        coord = out[0];
        if coord.0 == last_row {
            return len;
        }
        out.clear();
        get_next(grid, coord, &mut out);
        len +=1;
        out.retain(|next| *next != previous);
    }
    let Some(max_len) = out.iter().map(|next| longest_path(grid, *next, last_row)).max() else {return 0;};
    max_len+len
}

fn get_next_no_slopes(grid: &Vec<Vec<char>>, coord: (usize,usize), out: &mut Vec<(usize,usize)>) {
    let (row,col) = coord;
    if row != 0 && grid[row-1][col] != '#' {
        out.push((row-1, col));
    }
    if grid[row+1][col] != '#' {
        out.push((row+1, col));
    }
    if grid[row][col-1] != '#' {
        out.push((row, col-1));
    }
    if grid[row][col+1] != '#' {
        out.push((row, col+1));
    }
}

fn get_next(grid: &Vec<Vec<char>>, coord: (usize,usize), out: &mut Vec<(usize,usize)>) {
    let (row,col) = coord;
    if grid[row][col] != '.' {
        match grid[row][col] {
            '>' => out.push((row, col+1)),
            '<' => out.push((row, col-1)),
            '^' => out.push((row-1, col)),
            'v' => out.push((row+1, col)),
            ch  => panic!("Unexpected char {ch}"),
        }
        return;
    }
    if grid[row-1][col] != '#' && grid[row-1][col] != 'v' {
        out.push((row-1, col));
    }
    if grid[row+1][col] != '#' && grid[row+1][col] != '^' {
        out.push((row+1, col));
    }
    if grid[row][col-1] != '#' && grid[row][col-1] != '>' {
        out.push((row, col-1));
    }
    if grid[row][col+1] != '#' && grid[row][col+1] != '<' {
        out.push((row, col+1));
    }
}
