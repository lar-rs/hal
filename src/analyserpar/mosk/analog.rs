
/// Analog Mosk
///
///

use super::common::Generic;
use super::error::MockError;

use crate::common::Generic;
use crate::error::MockError;

/// SPI Transaction mode
#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    /// Write transaction
    Write,
    /// Write and read transaction
    Transfer,
    /// Send transaction
    Send,
    /// After a send transaction in real HW a Read is available
    Read,
}


