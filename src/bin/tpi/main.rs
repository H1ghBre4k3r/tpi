use std::error::Error;

use turing_pi::nodes::NODE_1;

fn main() -> Result<(), Box<dyn Error>> {
    let power = NODE_1.get_power()?;

    println!("Power of node 1: {power:#?}");
    Ok(())
}
