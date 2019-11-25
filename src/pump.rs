/// Lamp UV

use embedded_hal::digital::v2::OutputPin;
// use embedded_hal::timer::CountDown;
// use embedded_hal::timer::Periodic;
// use nb::block;
use super::error::Error;
/// Beeper
pub struct Pump<PIN>
where
    PIN: OutputPin,
{
    /// pin
    pin: PIN,
    // rt:  u64,
    // st:  u64,
}

impl<PIN> Pump<PIN>
where
    PIN: OutputPin,
{
    pub fn create(pin: PIN) -> Self {
        Pump { pin }
    }

    pub fn start(&mut self) -> nb::Result<(),Error> {
        self.pin.set_high().ok();
        Ok(())
    }
    pub fn stop(&mut self) -> nb::Result<(),Error>  {
        self.pin.set_low().ok();
        Ok(())
    }
}
