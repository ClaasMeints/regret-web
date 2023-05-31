
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use crate::graph::node::Node;
use crate::graph::component::Component;

pub struct AdjacencyMatrix {
    // the matrix is in triangular form -> no redundant information (also no diagonal)
    matrix: HashMap<usize, HashMap<usize, bool>>,
    default_vector: HashMap<usize, bool>,
    max_index: usize,
}

impl AdjacencyMatrix {
    fn new() -> Self {
        Self {
            matrix: HashMap::new(),
            default_vector: HashMap::new(),
            max_index: 0,
        }
    }

    fn add_terminal(&mut self, id: usize) {
        self.matrix.insert(id, self.default_vector.clone());
        self.default_vector.insert(id, false);
        self.max_index += 1;
    }

    fn remove_terminal(&mut self, id: &usize) {
        self.matrix.remove(id);
        for row in self.matrix.values_mut() {
            row.remove(id);
        }
        self.default_vector.remove(id);
    }

    fn set_value(&mut self, id_1: &usize, id_2: &usize, value: bool) -> Result<(), String> {
        // only set for the longer row to preserve triangular form
        if let Some(row) = self.matrix.get_mut(id_1) {
            if let Some(entry) = row.get_mut(id_2) {
                // terminal 1 is longer
                *entry = value;
                return Ok(());
            }
        }
        if let Some(row) = self.matrix.get_mut(id_2) {
            if let Some(entry) = row.get_mut(id_1) {
                // terminal 2 is longer
                *entry = value;
                return Ok(());
            }
        }
        Err("Could not set value".to_string())
    }

    fn add_connection(&mut self, id_1: &usize, id_2: &usize) {
        self.set_value(id_1, id_2, true);
    }

    fn remove_connection(&mut self, id_1: &usize, id_2: &usize) {
        self.set_value(id_1, id_2, false);
    }

    fn connected_terminals(&self, id_1: &usize, skip_rows: &mut Vec<usize>) -> HashSet<usize> {
        let mut connected_terminals: HashSet<usize> = HashSet::new();
        if skip_rows.contains(id_1) {
            // already connected to one node -> impossible to be connected to another
            return connected_terminals;
        }
        // skip this row in the future
        skip_rows.push(*id_1);
        // get all connected terminals (horizontal)
        if let Some(row) = self.matrix.get(id_1) {
            for (id_2, value) in row {
                if *value {
                    connected_terminals.insert(*id_2);
                }
            }
        }
        // get all connected terminals (vertical)
        for (id_2, row) in self.matrix.iter() {
            if let Some(value) = row.get(id_1) {
                if *value {
                    connected_terminals.insert(*id_2);
                }
            }
        }
        let mut connected_sub_terminals: Vec<usize> = Vec::new();
        for id_1 in connected_terminals.iter() {
            // recursively get all connected terminals and mark them as skipped
            connected_sub_terminals.extend(self.connected_terminals(id_1, skip_rows));
        }
        connected_terminals.extend(connected_sub_terminals);
        connected_terminals
    }

    fn create_nodes(&self) -> Vec<Node> {
        let mut nodes: Vec<Node> = Vec::new();
        let mut skip_rows: Vec<usize> = Vec::new();
        let mut node_index = 0;
        for row_index in (1..=self.max_index).rev() {
            if skip_rows.contains(&row_index) {
                continue;
            }
            let connected_terminals = self.connected_terminals(&row_index, &mut skip_rows);
            if !connected_terminals.is_empty() {
                nodes.push(Node::new(node_index, connected_terminals));
                node_index += 1;
            }
        }
        nodes
    }
}

impl fmt::Display for AdjacencyMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Terminal ___:")?;
        for column_index in 1..=self.max_index {
            write!(f, " {:3}", column_index)?;
        }
        writeln!(f)?;
        // enumerate HashMap
        for row_index in 1..=self.max_index {
            write!(f, "Terminal {:3}:", row_index)?;
            if let Some(terminal_vector) = self.matrix.get(&row_index) {
                for column_index in 1..=self.max_index {
                    if let Some(entry) = terminal_vector.get(&column_index) {
                        write!(f, " {:3}", *entry as usize)?;
                    } else {
                        write!(f, "    ")?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
