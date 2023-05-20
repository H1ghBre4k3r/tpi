use std::{thread::sleep, time::Duration};

use crate::{
    gpio::{
        pins::{POWER_EN, SYS_LED},
        GpioResult, GpioState,
    },
    nodes::{NodePower, NODE_1, NODE_2, NODE_3, NODE_4},
};

pub struct ATX;

impl ATX {
    pub fn get_power() -> GpioResult<NodePower> {
        Ok(POWER_EN.read()?.is_high().into())
    }

    pub fn power_on() -> GpioResult<()> {
        POWER_EN.write(GpioState::High)?;

        sleep(Duration::from_millis(100));

        SYS_LED.write(GpioState::Low)?;

        Ok(())
    }

    pub fn power_off() -> GpioResult<()> {
        POWER_EN.write(GpioState::Low)?;

        sleep(Duration::from_millis(100));

        SYS_LED.write(GpioState::High)?;

        Ok(())
    }
}

pub struct Board;

impl Board {
    /// Power on the entire board.
    pub fn power_on() -> GpioResult<()> {
        ATX::power_on()?;

        NODE_1.power_on()?;
        sleep(Duration::from_secs(1));

        NODE_2.power_on()?;
        sleep(Duration::from_secs(1));

        NODE_3.power_on()?;
        sleep(Duration::from_secs(1));

        NODE_4.power_on()?;
        sleep(Duration::from_secs(1));

        Ok(())
    }

    /// Power off the entire board.
    pub fn power_off() -> GpioResult<()> {
        NODE_4.power_on()?;
        sleep(Duration::from_secs(1));

        NODE_3.power_on()?;
        sleep(Duration::from_secs(1));

        NODE_2.power_on()?;
        sleep(Duration::from_secs(1));

        NODE_1.power_on()?;
        sleep(Duration::from_secs(1));

        ATX::power_off()?;

        Ok(())
    }
}
