use std::fs::File;
use std::io::prelude::*;

use super::{GpioError, GpioState};

#[derive(Debug, Clone, Copy)]
pub enum GpioPin {
    PB(u8),
    PC(u8),
    PD(u8),
    PE(u8),
    PF(u8),
    PG(u8),
}

impl From<GpioPin> for u8 {
    fn from(value: GpioPin) -> Self {
        match value {
            GpioPin::PB(x) => x + 32,
            GpioPin::PC(x) => x + 64,
            GpioPin::PD(x) => x + 96,
            GpioPin::PE(x) => x + 128,
            GpioPin::PF(x) => x + 160,
            GpioPin::PG(x) => x + 192,
        }
    }
}

pub const RTL_RESET: GpioPin = GpioPin::PG(13);
pub const SYS_LED: GpioPin = GpioPin::PG(8);
pub const RESET_LED: GpioPin = GpioPin::PG(9);
pub const SYS_RESET: GpioPin = GpioPin::PG(11);
pub const POWER_DETECT: GpioPin = GpioPin::PG(10);
pub const POWER_BOARD: GpioPin = GpioPin::PG(15);

pub const POWER_EN: GpioPin = GpioPin::PD(3);
pub const MODE1_EN: GpioPin = GpioPin::PD(7);
pub const MODE2_EN: GpioPin = GpioPin::PD(6);
pub const MODE3_EN: GpioPin = GpioPin::PD(5);
pub const MODE4_EN: GpioPin = GpioPin::PD(4);

pub const PORT1_EN: GpioPin = GpioPin::PD(11);
pub const PORT2_EN: GpioPin = GpioPin::PD(10);
pub const PORT3_EN: GpioPin = GpioPin::PD(9);
pub const PORT4_EN: GpioPin = GpioPin::PD(8);

pub const PORT1_RST: GpioPin = GpioPin::PD(0);
pub const PORT2_RST: GpioPin = GpioPin::PD(20);
pub const PORT3_RST: GpioPin = GpioPin::PD(21);
pub const PORT4_RST: GpioPin = GpioPin::PD(22);

pub const PORT1_USB_VBUS: GpioPin = GpioPin::PD(19);
pub const PORT2_USB_VBUS: GpioPin = GpioPin::PD(18);
pub const PORT3_USB_VBUS: GpioPin = GpioPin::PD(17);
pub const PORT4_USB_VBUS: GpioPin = GpioPin::PD(16);

pub const PORT1_RPIBOOT: GpioPin = GpioPin::PD(15);
pub const PORT2_RPIBOOT: GpioPin = GpioPin::PD(14);
pub const PORT3_RPIBOOT: GpioPin = GpioPin::PD(12);
pub const PORT4_RPIBOOT: GpioPin = GpioPin::PD(13);

pub const USB_SEL1: GpioPin = GpioPin::PG(1);
pub const USB_SEL2: GpioPin = GpioPin::PG(0);
pub const USB_OE1: GpioPin = GpioPin::PG(2);
pub const USB_OE2: GpioPin = GpioPin::PG(3);

pub const USB_SWITCH: GpioPin = GpioPin::PG(5);
pub const USB_PWEN: GpioPin = GpioPin::PG(4);

impl GpioPin {
    pub fn read(&self) -> Result<GpioState, GpioError> {
        let gpio_path = format!("/sys/class/gpio/gpio{}/value", u8::from(*self));

        let Ok(mut file) = File::open(&gpio_path) else {
            return Err(GpioError(format!(
                "Could not open file '{gpio_path}'"
            )));
        };

        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return Err(GpioError(format!(
                "Could not read contents of file '{gpio_path}'"
            )));
        }

        let gpio_value = contents.chars().next().map_or('0', |v| v);

        Ok(gpio_value.into())
    }

    pub fn write(&self, value: GpioState) -> Result<(), GpioError> {
        let gpio_path = format!("/sys/class/gpio/gpio{}/value", u8::from(*self));

        if std::fs::write(&gpio_path, format!("{}", u8::from(value))).is_err() {
            return Err(GpioError(format!("Could not write to file '{gpio_path}'")));
        };

        Ok(())
    }
}
