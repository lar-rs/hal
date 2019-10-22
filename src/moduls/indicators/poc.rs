/// Suspended Organic Carbon – also called particulate organic carbon (POC); the carbon in particulate form that is too large to pass through a filter.

use serde::{Deserialize, Serialize};
// use super::channel::{Channel};




 #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct POC{
    value: f64,
}
