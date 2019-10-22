
/// Monitor gear pump normally used for solution sampling. 
/// 
/// 

pub trait Control{

    pub fn start(&mut self,v:f64);
    pub fn stop(&mut self,v:f64);
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlowRate {
    pwm : u32;
}

pub trait Pump {
    
    type Error;
    pub fn run( &mut self)-> Result<(), Self::Error>;
    pub fn stop( &mut self)->Result<(), Self::Error>;
    pub fn set_flow_rate(&mut self, rate:FlowRate)->Result<(), Self::Error>;
}


