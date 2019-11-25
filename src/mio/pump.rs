/// Monitor gear pump normally used for solution sampling.
///
///
// use async_std::prelude::*;
use async_std::io::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Pump {
    async fn start(&mut self)  -> Result<()>;
    async fn stop(&mut self) -> Result<()>;
}





// #[cfg(feature = "mosk")]
pub mod mosk {
    use async_std::prelude::*;
    use async_std::stream;
    use std::time::{Duration};
    use async_trait::async_trait;
    use async_std::io::Result;

    /// Lamp simulation
    pub struct Pump {
        on: bool,
    }

    #[async_trait]
    impl super::Pump for Pump {
        async fn start(&mut self)  -> Result<()> {
            if !self.on{
                let mut interval  = stream::interval(Duration::from_millis(250));
                interval.next().await;
                self.on = true;
            }
            Ok(())
        }
        async fn stop(&mut self) -> Result<()> {
            if self.on {
                let mut interval  = stream::interval(Duration::from_millis(250));
                interval.next().await;
                self.on = false;
            }
            Ok(())
        }
    }
}

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
// }
