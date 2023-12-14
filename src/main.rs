use std::env;
use std::fs::File;
use csv::{ReaderBuilder};
use std::collections::{VecDeque};
use std::io;

mod player;
fn read_csv_file(file_path: &str) -> Vec<player::Player> {
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

fn edges(nodes: Vec<player::Player>, n: usize) -> Vec<Vec<usize>> {
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

fn bfs_with_path(nodes: &[player::Player], edges: &[Vec<usize>], start: &str, end: &str) -> Option<Vec<String>> {
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

fn reconstruct_path(nodes: &[player::Player], parents: Vec<Option<usize>>, start: usize, end: usize) -> Vec<String> {
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

fn dist_test(nodes: &Vec<player::Player>, edges: &Vec<Vec<usize>>) {
    if let Some(path) = bfs_with_path(nodes, edges, "L. Messi", "R. Lewandowski") {
        assert_eq!(path.len(), 4, "Distance computation inaccurate");
    } else {
        println!("Error: Distance does not exist.")
    }
}

fn dist_test2(nodes: &Vec<player::Player>, edges: &Vec<Vec<usize>>) {
    if let Some(path) = bfs_with_path(nodes, edges, "L. Messi", "G. Donnarumma") {
        assert_eq!(path.len(), 2, "Distance computation inaccurate");
    } else {
        println!("Error computing BFS")
    }
}

fn main() {
    println!("Please wait...");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path/to/your/file.csv>", args[0]);
        std::process::exit(1);
    }
    let _file_path = &args[1];
    let nodes = read_csv_file(_file_path);
    let neighbors = edges(nodes.clone(), nodes.len());
    dist_test(&nodes, &neighbors);
    dist_test2(&nodes, &neighbors);
    loop {
        println!("Enter a player name:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input_name = input.trim();
        match player::find_player(&nodes, input_name) {
            Some(starting_player) => {
                println!("Player found, {:?}", starting_player);
                    loop {
                        println!("What player distance would you like to see? Enter a player name:");
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let input_name = input.trim();
                        match player::find_player(&nodes, input_name) {
                            Some(player) => {
                                println!("Player found, {:?}", player); {
                                    if let Some(path) = bfs_with_path(&nodes, &neighbors, starting_player.name.as_str(), player.name.as_str()) {
                                        println!("The path between {:?} and {:?} is {:?}", starting_player.name, player.name, path);
                                        break;
                                    } else {
                                        println!("No path found between {:?} and {:?}", starting_player.name, player.name);
                                        break;
                                    }
                                }
                            }
                            None => {
                                println!("Player not found. Please try again.");
                            }
                        }
                    }
                    break;
                }
            None => todo!(),
            }
        }
    }







