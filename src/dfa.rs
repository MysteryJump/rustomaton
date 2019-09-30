use super::edge;
use super::vertex;

pub struct DFA<'a> {
    pub states_count: u32,
    pub functions_count: u32,
    pub states: &'a mut Vec<vertex::Vertex>,
    pub functions: &'a mut Vec<edge::Edge>,
    pub starts_index: u32,
    pub acceptance_state: &'a mut Vec<u32>,
    pub acceptance_state_count: u32,
}

impl<'a> DFA<'a> {
    pub fn initialize_dfa(
        states: &'a mut Vec<vertex::Vertex>,
        functions: &'a mut Vec<edge::Edge>,
        start_index: u32,
        acceptace_indexes: &'a mut Vec<u32>,
    ) -> DFA<'a> {
        let states_len = states.len() as u32;
        let functions_len = functions.len() as u32;
        let acceptaces_len = acceptace_indexes.len() as u32;
        let dfa = DFA {
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
    pub fn check_accept(&self, target_txt: String) -> bool {
        let mut current_state = self.starts_index;
        let mut text = target_txt.clone().chars().rev().collect::<String>();
        let mut grid: Vec<Vec<Vec<char>>> = Vec::with_capacity(self.states_count as usize);
        for _ in 0..self.states_count {
            let mut vec: Vec<Vec<char>> = Vec::with_capacity(self.states_count as usize);
            for _ in 0..self.states_count {
                vec.push(Vec::new());
            }
            grid.push(vec);
        }
        for item in self.functions.iter() {
            grid[item.from as usize][item.to as usize] = item.tag.clone();
        }
        loop {
            let mut is_tansitioned = false;
            if text.len() == 0 {
                return self.acceptance_state.contains(&current_state);
            }
            let now_char: char = text.pop().unwrap();
            for i in 0..self.states_count {
                let grid_chars = grid[current_state as usize][i as usize].clone();
                for chr in grid_chars {
                    if chr == now_char {
                        current_state = i;
                        is_tansitioned = true;
                        break;
                    }
                }
            }
            if !is_tansitioned {
                return false;
            }
        }
    }
}
