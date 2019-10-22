//! `TC`
//! Wiki: https://en.wikipedia.org/wiki/Total_organic_carbon
//! * Total Carbon (TC) – all the carbon in the sample, including both inorganic and organic carbon
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Channel{
    timestamp: u64,
    fsr: f64,
    value: f64,
}


pub struct Measurement {

}


pub trait TC {

}
