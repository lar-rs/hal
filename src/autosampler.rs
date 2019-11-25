
/// Autosampler
///
///
///
use nb;

pub trait Autosampler{
    type Error;
    type Position;
    type Speed;

    fn hold(&mut self) -> nb::Result<Self::Position,Self::Error>;
    fn position(&mut self, pos:Self::Position) -> nb::Result<Self::Position,Self::Error>;
    fn take(&mut self, par:Self::Speed) -> nb::Result<(),Self::Error>;
    fn push(&mut self, par:Self::Speed) -> nb::Result<(),Self::Error>;
}


#[cfg(feature = "mosk")]
pub mod mosk {
    use crate::error::MockError;
    use crate::common::Generic;


// /// Analog Out mock
// ///
// /// Models an Analog read or write
// #[derive(Clone, Debug, PartialEq)]
// pub struct MoskAnalogOut{
//     value: f32;
// }


// impl Analog for MoskAnalogOut {
//     type Error = MockError;
//     type Value = f32;

//     fn set_value (&mut self, v : Value) ->Result<(), Self::Error> {
//         self.value = v;
//         Ok(())
//     }
}


