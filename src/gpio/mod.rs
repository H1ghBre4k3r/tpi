pub mod error;
pub mod pins;
pub mod state;

pub use self::error::*;
pub use self::state::*;

pub type GpioResult<T> = Result<T, GpioError>;
