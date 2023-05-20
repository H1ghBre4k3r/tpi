#[derive(Debug, Clone, Copy)]
pub enum GpioState {
    High,
    Low,
}

impl GpioState {
    pub fn is_high(&self) -> bool {
        match self {
            GpioState::High => true,
            GpioState::Low => false,
        }
    }

    pub fn is_low(&self) -> bool {
        match self {
            GpioState::High => false,
            GpioState::Low => true,
        }
    }
}

impl From<char> for GpioState {
    fn from(value: char) -> Self {
        use GpioState::*;

        if value == '0' {
            Low
        } else {
            High
        }
    }
}

impl From<GpioState> for u8 {
    fn from(value: GpioState) -> Self {
        use GpioState::*;
        match value {
            High => 1,
            Low => 0,
        }
    }
}
