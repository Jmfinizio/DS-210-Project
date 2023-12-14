use std::fs::File;
use csv::{ReaderBuilder};
use std::collections::{VecDeque};

pub mod player;

pub fn read_csv_file(file_path: &str) -> Vec<player::Player> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening CSV file: {}", e);
            return Vec::new();
        }
    };
    let mut rdr = ReaderBuilder::new().delimiter(b',').flexible(true).has_headers(true).from_reader(file);
    let mut nodes: Vec<player::Player> = Vec::new();
    for result in rdr.deserialize::<player::Player>() {
        match result {
            Ok(record) => {
                nodes.push(record);
            }
            Err(e) => eprintln!("Error reading CSV record: {}", e),
        }
    }

    nodes
}

pub fn edges(nodes: Vec<player::Player>, n: usize) -> Vec<Vec<usize>> {
    let mut edges: Vec<Vec<usize>> = vec![vec![]; n];
    for (i, player) in nodes.iter().enumerate() {
        for (j, player2) in nodes.iter().enumerate() {
            if player.nation == player2.nation || player.team == player2.team {
                edges[i].push(j);
            }
        }
    }
    edges
}

pub fn bfs_with_path(nodes: &[player::Player], edges: &[Vec<usize>], start: &str, end: &str) -> Option<Vec<String>> {
    if let Some(start_index) = nodes.iter().position(|x| x.name == start) {
        if let Some(end_index) = nodes.iter().position(|x| x.name == end) {
            let mut distances: Vec<Option<u32>> = vec![None; edges.len()];
            let mut parents: Vec<Option<usize>> = vec![None; edges.len()];
            distances[start_index] = Some(0);
            let mut queue: VecDeque<usize> = VecDeque::new();
            queue.push_back(start_index);
            while let Some(v) = queue.pop_front() {
                for &u in &edges[v] {
                    if distances[u].is_none() {
                        distances[u] = Some(distances[v].unwrap() + 1);
                        parents[u] = Some(v);
                        queue.push_back(u);
                        if u == end_index {
                            return Some(reconstruct_path(nodes, parents, start_index, end_index));
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn reconstruct_path(nodes: &[player::Player], parents: Vec<Option<usize>>, start: usize, end: usize) -> Vec<String> {
    let mut path = vec![nodes[end].name.clone()];
    let mut current = end;

    while let Some(parent) = parents[current] {
        path.push(nodes[parent].name.clone());
        current = parent;
        if current == start {
            break;
        }
    }
    path.reverse();
    path
}