mod dfa;
mod edge;
mod io;
mod vertex;

use std::io::Write;

fn main() {
    // let args : Vec<String> = std::env::args().collect();
    let v_count: u32 = io::read_count("How much vertexs in the Graph?".to_string());
    let e_count: u32 = io::read_count("How much transition functions in the Graph?".to_string());

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
    let dfa = dfa::DFA::initialize_dfa(
        &mut vertexs,
        &mut edges,
        start_vertex_index,
        &mut end_indexes,
    );
    let is_accepted = dfa.check_accept(input_line.trim_end().to_string());

    match is_accepted {
        true => println!("Accepted :)"),
        false => println!("Not Accepted :("),
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
        let from_state = io::read_count(format!("{}'s from state: ", i));
        let to_state = io::read_count(format!("{}'s to state: ", i));

        let input_character =
            io::read_characters(format!("{}'s input characters (space ignored): ", i));
        edges.push(edge::Edge {
            index: i,
            from: from_state,
            to: to_state,
            tag: input_character,
        });
    }
    edges
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

#[test]
fn check_accept_test_1() {
    let mut states = vec![
        vertex::Vertex {
            index: 0,
            name: "0".to_string(),
        },
        vertex::Vertex {
            index: 1,
            name: "1".to_string(),
        },
    ];
    let mut functions = vec![
        edge::Edge {
            index: 0,
            tag: vec!['a'],
            from: 0,
            to: 1,
        },
        edge::Edge {
            index: 1,
            tag: vec!['b'],
            from: 1,
            to: 0,
        },
    ];
    let mut ends = vec![1];
    let dfa = dfa::DFA::initialize_dfa(&mut states, &mut functions, 0, &mut ends);
    assert_eq!(dfa.check_accept("a".to_string()), true);
    assert_eq!(dfa.check_accept("ab".to_string()), false);
    assert_eq!(dfa.check_accept("ababa".to_string()), true);
}

#[test]
fn check_accept_test_2() {
    let mut states = vec![
        vertex::Vertex {
            index: 0,
            name: "0".to_string(),
        },
        vertex::Vertex {
            index: 1,
            name: "1".to_string(),
        },
        vertex::Vertex {
            index: 2,
            name: "2".to_string(),
        },
    ];
    let mut functions = vec![
        edge::Edge {
            index: 0,
            tag: vec!['1'],
            from: 0,
            to: 0,
        },
        edge::Edge {
            index: 1,
            tag: vec!['0'],
            from: 0,
            to: 1,
        },
        edge::Edge {
            index: 2,
            tag: vec!['0'],
            from: 1,
            to: 1,
        },
        edge::Edge {
            index: 3,
            tag: vec!['1'],
            from: 1,
            to: 2,
        },
        edge::Edge {
            index: 4,
            tag: vec!['0', '1'],
            from: 2,
            to: 2,
        },
    ];
    let mut ends = vec![2];
    let dfa = dfa::DFA::initialize_dfa(&mut states, &mut functions, 0, &mut ends);
    assert_eq!(dfa.check_accept("01".to_string()), true);
    assert_eq!(dfa.check_accept("11110".to_string()), false);
    assert_eq!(dfa.check_accept("aaaaa".to_string()), false);
    assert_eq!(dfa.check_accept("1111000110101010101101".to_string()), true);
}
