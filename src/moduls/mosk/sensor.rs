/// Sensor HAL 
/// NDir1,NDir2 Sauerstoff
/// 
/// 



pub trait Sensor {
    type Error;
    type Signal;

    fn signal(&mut self) -> Result<Self::Signal, Self::Error>;
}
