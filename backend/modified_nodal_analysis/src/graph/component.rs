
use num::complex::Complex64;

pub struct Component {
    name: String,
    terminal_ids: [usize; 2],
    value: Complex64,
}
