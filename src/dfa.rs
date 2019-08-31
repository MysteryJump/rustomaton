use super::vertex;
use super::edge;

pub struct DFA<'a> {
    pub states_count: u32,
    pub functions_count: u32,
    pub states: &'a mut Vec<vertex::Vertex>,
    pub functions: &'a mut Vec<edge::Edge>,
    pub starts_index: u32,
    pub acceptance_state: &'a mut Vec<u32>,
    pub acceptance_state_count: u32
}
