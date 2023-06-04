use num::complex::Complex64;

pub struct Component {
    name: String,
    terminal_ids: [usize; 2],
    value: Complex64,
}

impl Component {
    pub fn new(name: String, value: Complex64) -> Self {
        Self {
            name,
            terminal_ids: [0, 0],
            value,
        }
    }

    pub fn ids(&self) -> [usize; 2] {
        self.terminal_ids
    }
}
