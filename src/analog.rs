///
/// Analog
///
///
///


/// SPI Transaction mode
pub trait Analog{
    type Value;
    type Error;
    fn set_value(&mut self,v:Self::Value)->nb::Result<(),Self::Error>;
    fn get_value(&mut self)->nb::Result<Self::Value,Self::Error>;
}





#[cfg(feature = "mosk")]
pub mod mosk {
    use nb;
    use crate::error::MockError;
    use crate::common::Generic;


// Analog Out mock
//
// #[derive(Clone, Debug, PartialEq)]
// pub struct MoskAnalogOut{
    // value: f32;
// }


// impl Analog for MoskAnalogOut {
    // type Error = MockError;
    // type Value = f32;

    // fn set_value (&mut self, v : Value) ->Result<(), Self::Error> {
        // self.value = v;
        // Ok(())
    // }
}
