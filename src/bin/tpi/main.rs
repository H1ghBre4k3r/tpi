use std::error::Error;

use clap::{Parser, ValueEnum};
use turing_pi::nodes::{NodePower, NODE_1, NODE_2, NODE_3, NODE_4};

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// The node to interact with. Without '--power' this command will print useful node
    /// information.
    #[arg(short, long)]
    pub node: u8,

    /// By specifying this argument, you either power on or power off the specified node.
    #[arg(short, long)]
    pub power: Option<PowerLevel>,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum PowerLevel {
    /// Power the node on.
    #[value(alias("1"))]
    On,

    /// Power the node off.
    #[value(alias("0"))]
    Off,
}

impl From<PowerLevel> for NodePower {
    fn from(value: PowerLevel) -> Self {
        match value {
            PowerLevel::On => NodePower::On,
            PowerLevel::Off => NodePower::Off,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let node = match args.node {
        1 => NODE_1,
        2 => NODE_2,
        3 => NODE_3,
        4 => NODE_4,
        _ => unimplemented!("This application currently only supports nodes 1 - 4"),
    };

    if let Some(power) = args.power {
        node.set_power(power.into())?;
    } else {
        let power = node.get_power()?;
        println!(
            "Power status of node {node_id}: {power:?}",
            node_id = args.node
        );
    }

    Ok(())
}
