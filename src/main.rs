mod dfa;
mod edge;
mod vertex;

use std::io::Write;

fn main() {
    // let args : Vec<String> = std::env::args().collect();
    let v_count: u32 = read_count("How much vertexs in the Graph?".to_string());
    let e_count: u32 = read_count("How much transition functions in the Graph?".to_string());

    let mut vertexs = read_vertexs(v_count);
    let mut edges = read_edges(e_count);

    let start_vertex_index: u32 = read_start(&vertexs);
    let mut end_indexes = read_ends(&vertexs);

    let mut input_line = String::new();
    print!("Texts: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line.");
    let dfa = initialize_dfa(
        &mut vertexs,
        &mut edges,
        start_vertex_index,
        &mut end_indexes,
    );
    let is_accepted = check_accept(&dfa, input_line.trim_end().to_string());

    match is_accepted {
        true => println!("Accepted :)"),
        false => println!("Not Accepted :("),
    }
}

fn read_count(promt_str: String) -> u32 {
    let mut input_line = String::new();
    println!("{}", promt_str);
    loop {
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to readline.");
        let count: u32 = match input_line.trim_end().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input positive integer.");
                continue;
            }
        };
        return count;
    }
}

fn read_vertexs(v_count: u32) -> Vec<vertex::Vertex> {
    print!("Do you want to generate vertexs from vertex count? (Y/n) : ");
    std::io::stdout().flush().unwrap();
    let mut vertexs: Vec<vertex::Vertex> = Vec::with_capacity(v_count as usize);
    let mut input_line = String::new();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to readline.");
    let is_auto_generate = match input_line.trim_end() {
        "N" => false,
        "No" => false,
        "n" => false,
        _ => true,
    };
    match is_auto_generate {
        true => {
            for index in 0..v_count {
                vertexs.push(vertex::Vertex {
                    name: index.to_string(),
                    index: index,
                })
            }
        }
        false => {
            for index in 0..v_count {
                print!("{}'s Name: ", index);
                std::io::stdout().flush().unwrap();
                let mut name_line = String::new();
                std::io::stdin()
                    .read_line(&mut name_line)
                    .expect("Failed to readline.");
                vertexs.push(vertex::Vertex {
                    name: name_line.trim_end().to_string(),
                    index: index,
                });
            }
        }
    };
    vertexs
}

fn read_edges(e_count: u32) -> Vec<edge::Edge> {
    let mut edges: Vec<edge::Edge> = Vec::with_capacity(e_count as usize);
    for i in 0..e_count {
        let from_state = read_count(format!("{}'s from state: ", i));
        let to_state = read_count(format!("{}'s to state: ", i));

        let input_character = read_character(format!("{}'s input character: ", i));
        edges.push(edge::Edge {
            index: i,
            from: from_state,
            to: to_state,
            tag: input_character,
        });
    }
    edges
}

fn read_character(promt_str: String) -> char {
    let mut input_line: String = String::new();
    println!("{}", promt_str);
    loop {
        std::io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line.");
        if input_line.trim_end().len() == 1 {
            let x = input_line.chars().next().unwrap();
            return x;
        } else {
            println!("Please input one character.");
            continue;
        }
    }
}

fn read_start(vertexs: &Vec<vertex::Vertex>) -> u32 {
    print!("Which vertex is Start? (0-indexed index or name): ");
    std::io::stdout().flush().unwrap();
    let mut start_line = String::new();
    std::io::stdin()
        .read_line(&mut start_line)
        .expect("Failed to read line.");
    loop {
        let filtered_vertexs: Vec<_> = vertexs
            .iter()
            .clone()
            .filter(|&x| {
                x.name == start_line.trim_end()
                    || x.index == start_line.trim_end().parse().unwrap_or(2 << 31)
            })
            .collect();
        if filtered_vertexs.len() == 0 {
            println!("Please input existing value.");
            continue;
        }
        return (**filtered_vertexs.get(0).unwrap()).index;
    }
}

fn read_ends(vertexs: &Vec<vertex::Vertex>) -> Vec<u32> {
    println!("Which state is state of acceptance?");
    let mut acceptances: Vec<u32> = Vec::new();
    let mut input_line = String::new();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line.");
    let items = input_line.split(',');
    for item in items {
        let filtered_vertexs: Vec<_> = vertexs
            .iter()
            .clone()
            .filter(|&x| x.name == item.trim() || x.index == item.trim().parse().unwrap_or(2 << 31))
            .collect();
        if filtered_vertexs.len() == 0 {
            println!("Value {} is thorughted", item);
            continue;
        }
        acceptances.push((**filtered_vertexs.get(0).unwrap()).index);
    }
    acceptances
}

fn initialize_dfa<'a>(
    states: &'a mut Vec<vertex::Vertex>,
    functions: &'a mut Vec<edge::Edge>,
    start_index: u32,
    acceptace_indexes: &'a mut Vec<u32>,
) -> dfa::DFA<'a> {
    let states_len = states.len() as u32;
    let functions_len = functions.len() as u32;
    let acceptaces_len = acceptace_indexes.len() as u32;
    let dfa = dfa::DFA {
        starts_index: start_index,
        states: states,
        states_count: states_len,
        functions: functions,
        functions_count: functions_len,
        acceptance_state: acceptace_indexes,
        acceptance_state_count: acceptaces_len,
    };
    dfa
}

fn check_accept(dfa: &dfa::DFA, target_txt: String) -> bool {
    let mut current_state = dfa.starts_index;
    let mut text = target_txt.clone().chars().rev().collect::<String>();
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(dfa.states_count as usize);
    for _ in 0..dfa.states_count {
        let mut vec: Vec<char> = Vec::with_capacity(dfa.states_count as usize);
        for _ in 0..dfa.states_count {
            vec.push(' ');
        }
        grid.push(vec);
    }
    for item in dfa.functions.iter() {
        grid[item.from as usize][item.to as usize] = item.tag;
    }
    loop {
        let mut is_tansitioned = false;
        if text.len() == 0 {
            return dfa.acceptance_state.contains(&current_state);
        }
        let now_char: char = text.pop().unwrap();
        for i in 0..dfa.states_count {
            if grid[current_state as usize][i as usize] == now_char {
                current_state = i;
                is_tansitioned = true;
                break;
            }
        }
        if !is_tansitioned {
            return false;
        }
    }
}

#[test]
fn check_accept_test() {
    let mut states = vec![
        vertex::Vertex {
            index: 0,
            name: "0".to_string()
        },
        vertex::Vertex {
            index: 1,
            name: "1".to_string()
        }
    ];
    let mut functions = vec![
        edge::Edge {
            index: 0,
            tag: 'a',
            from: 0,
            to: 1
        },
        edge::Edge {
            index: 1,
            tag: 'b',
            from: 1,
            to: 0
        }
    ];
    let mut ends = vec![1];
    let dfa = initialize_dfa(&mut states, &mut functions, 0, &mut ends);
    assert_eq!(check_accept(&dfa, "a".to_string()),true);
    assert_eq!(check_accept(&dfa, "ab".to_string()),false);
    assert_eq!(check_accept(&dfa, "ababa".to_string()),true);

}
