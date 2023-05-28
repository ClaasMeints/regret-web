use symbolic_manipulation::*;

fn main() {
    let mut expression = expr!(Subtraction, expr!(2), expr!(3));
    if let Ok(result) = &mut expression {
        result.simplify();
        result.simplify();
        println!("{}", result);
    }
}
