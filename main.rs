use std::io::{self, BufRead, Write};
mod sacrificeCalculator;
fn main() {
    println!("Hello, world!");
    let stdio = io::stdin();
    let input = stdio.lock();

    let output = io::stdout();

    sacrificeCalculator::SacCalc(
        input,
        output,
        "Enter the cost of protective, conceptual, fundamental and obscure \n",
    );
}
