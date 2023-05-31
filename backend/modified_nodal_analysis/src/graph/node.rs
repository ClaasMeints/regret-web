use num::complex::Complex64;
use std::collections::HashSet;
pub struct Node {
    id: usize,
    terminal_ids: HashSet<usize>,
    voltage: Option<Complex64>,
}

impl Node {
    fn new(id: usize, terminal_ids: HashSet<usize>) -> Self {
        Self {
            id,
            terminal_ids,
            voltage: None,
        }
    }

    fn is_attached(&self, terminal_id: &usize) -> bool {
        self.terminal_ids.contains(terminal_id)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node {:3}:", self.id)?;
        for terminal_id in self.terminal_ids.iter() {
            write!(f, " {:3}", terminal_id)?;
        }
        Ok(())
    }
}
