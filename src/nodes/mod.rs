use std::{thread::sleep, time::Duration};

use crate::gpio::{
    pins::{
        GpioPin, MODE1_EN, MODE2_EN, MODE3_EN, MODE4_EN, PORT1_EN, PORT1_RST, PORT2_EN, PORT2_RST,
        PORT3_EN, PORT3_RST, PORT4_EN, PORT4_RST,
    },
    GpioError, GpioState,
};

#[derive(Debug, Clone)]
pub struct Node {
    mode: GpioPin,
    port: GpioPin,
    reset: GpioPin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodePower {
    On,
    Off,
}

impl From<bool> for NodePower {
    fn from(value: bool) -> Self {
        use NodePower::*;
        if value {
            On
        } else {
            Off
        }
    }
}

pub const NODE_1: Node = Node {
    mode: MODE1_EN,
    port: PORT1_EN,
    reset: PORT1_RST,
};

pub const NODE_2: Node = Node {
    mode: MODE2_EN,
    port: PORT2_EN,
    reset: PORT2_RST,
};

pub const NODE_3: Node = Node {
    mode: MODE3_EN,
    port: PORT3_EN,
    reset: PORT3_RST,
};

pub const NODE_4: Node = Node {
    mode: MODE4_EN,
    port: PORT4_EN,
    reset: PORT4_RST,
};

impl Node {
    pub fn get_power(&self) -> Result<NodePower, GpioError> {
        Ok((self.mode.read()?.is_high() && self.port.read()?.is_low()).into())
    }

    pub fn set_power(&self, power: NodePower) -> Result<(), GpioError> {
        if power == NodePower::Off {
            self.port.write(GpioState::High)?;
            sleep(Duration::from_millis(100));
            self.mode.write(GpioState::Low)?;

            if 118u8 != self.reset.into() {
                sleep(Duration::from_millis(100));
                self.reset.write(GpioState::Low)?;
            }
        } else {
            self.port.write(GpioState::Low)?;
            sleep(Duration::from_millis(100));
            self.mode.write(GpioState::High)?;

            if 118u8 != self.reset.into() {
                sleep(Duration::from_millis(100));
                self.reset.write(GpioState::High)?;
            }
        }

        Ok(())
    }

    pub fn power_on(&self) -> Result<(), GpioError> {
        self.set_power(NodePower::On)
    }

    pub fn power_off(&self) -> Result<(), GpioError> {
        self.set_power(NodePower::Off)
    }
}
