/// Valve I/O
///
///
use embedded_hal::digital::v2::OutputPin;
// use embedded_hal::timer::CountDown;
// use embedded_hal::timer::Periodic;
// use nb::block;
use super::error::Error;
/// Beeper
pub struct Valve<PIN>
where
    PIN: OutputPin,
{
    /// pin on/off
    pin: PIN,
}

impl<PIN> Valve<PIN>
where
    PIN: OutputPin,
{
    pub fn create(pin: PIN) -> Self {
        Valve { pin }
    }
    pub fn open(&mut self) -> nb::Result<(),Error> {
        self.pin.set_high().ok();
        Ok(())
    }
    pub fn close(&mut self) -> nb::Result<(),Error>  {
        self.pin.set_low().ok();
        Ok(())
    }
}
