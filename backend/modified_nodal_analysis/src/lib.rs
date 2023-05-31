use core::fmt;
use num::complex::Complex64;
use std::{collections::HashMap, sync::atomic::AtomicUsize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Terminal {
    id: usize,
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Terminal {
    fn new() -> Self {
        Self {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }
}

struct Component {
    name: String,
    terminals: [Terminal; 2],
    value: Complex64,
}

impl Component {
    fn new(name: String, value: Complex64) -> Self {
        let terminals = [Terminal::new(), Terminal::new()];
        Self {
            name,
            terminals,
            value,
        }
    }
}

struct AdjacencyMatrix {
    // the matrix is in triangular form -> no redundant information (also no diagonal)
    matrix: HashMap<usize, HashMap<usize, bool>>,
    default_vector: HashMap<usize, bool>,
}

impl AdjacencyMatrix {
    fn new() -> Self {
        Self {
            matrix: HashMap::new(),
            default_vector: HashMap::new(),
        }
    }

    fn add_terminal(&mut self, id: usize) {
        self.matrix.insert(id, self.default_vector.clone());
        self.default_vector.insert(id, false);
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
}

impl fmt::Display for AdjacencyMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Terminal ___:")?;
        for i in 1..self.matrix.len() {
            write!(f, " {:3} ", i)?;
        }
        writeln!(f)?;
        // enumerate HashMap
        for (index, terminal_vector) in self.matrix.values().enumerate() {
            write!(f, "Terminal {:3}:", index + 1)?;
            for entry in terminal_vector.values() {
                write!(f, " {:3}", *entry as usize)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

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
        for terminal in component.terminals.iter() {
            self.adjacency_matrix.add_terminal(terminal.id)
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
        print!("{}", circuit);
        circuit.add_component();
        println!("{}", circuit);
    }
}
