use core::fmt;
use num::complex::Complex64;
use std::collections::{HashMap, HashSet};

mod graph;
use crate::graph::*;

struct Circuit {
    components: Vec<Component>,
    adjacency_matrix: AdjacencyMatrix,
}

impl Circuit {
    fn new() -> Self {
        Self {
            components: Vec::new(),
            adjacency_matrix: AdjacencyMatrix::new(),
        }
    }

    fn add_component(&mut self) {
        let component = Component::new(String::from("R"), Complex64::new(1.0, 0.0));
        for id in component.terminal_ids.iter() {
            self.adjacency_matrix.add_terminal(*id);
        }
        self.components.push(component);
    }
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:}", self.adjacency_matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let mut circuit = Circuit::new();

        circuit.add_component();
        circuit.add_component();
        circuit.add_component();
        circuit.add_component();

        // circuit.adjacency_matrix.remove_terminal(&3);

        circuit.adjacency_matrix.add_connection(&1, &2);
        circuit.adjacency_matrix.add_connection(&1, &5);
        circuit.adjacency_matrix.add_connection(&3, &7);
        circuit.adjacency_matrix.add_connection(&7, &8);

        println!("{}", circuit);

        let nodes = circuit.adjacency_matrix.create_nodes();
        for node in nodes.iter() {
            println!("{}", node);
        }
    }
}
