use std::env;
use std::io;
use bfs::player;

mod bfs;

fn dist_test(nodes: &Vec<player::Player>, edges: &Vec<Vec<usize>>) {
    if let Some(path) = bfs::bfs_with_path(nodes, edges, "L. Messi", "R. Lewandowski") {
        assert_eq!(path.len(), 4, "Distance computation inaccurate");
    } else {
        println!("Error: Distance does not exist.")
    }
}

fn dist_test2(nodes: &Vec<player::Player>, edges: &Vec<Vec<usize>>) {
    if let Some(path) = bfs::bfs_with_path(nodes, edges, "L. Messi", "G. Donnarumma") {
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
    let nodes = bfs::read_csv_file(_file_path);
    let neighbors = bfs::edges(nodes.clone(), nodes.len());
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
                                    if let Some(path) = bfs::bfs_with_path(&nodes, &neighbors, starting_player.name.as_str(), player.name.as_str()) {
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







