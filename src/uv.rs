//! QuickTOCuv analyzer
//!
//! Kalibrierparameter
//!
//! Füllzeit Kalibrierlösung		???							Vorlaufzeit bis Lösung tatsächlich stabil bzw. t100 erreicht ist
//! Füllzeit Nullgas		???							Füllzeit Trägergas Detektor für Autokalibrierung
//! Signalabstand Kalibrierung		60							Messzeit für eine Signalaufnahme
//! Anzahl Messsignale pro Messung		3							Anzahl der Signalaufnahmen pro Messwiederholung
//! Messwiederholungen		5							Anzahl der Messwiederholungen pro Kalibrierlösung
//!
//! Messparameter – Online 		        PS1	PS2	PS3	PS4	PS5	PS6
//!
//! Messintervall								                    Wie definieren wir das für bis zu 6PS ?
//! Measurement delay     	        	0	0	0	0	0	0		startet die Messung Status M1 bsplw. Um irgendetwas anzutriggern
//! Füllzeit UV-Reaktor		           ???	???	???	???	???	???		beschreibt im Grunde die t90- oder t100 Zeit für Tendenzänderung auch kürzer
//! Zeit für Signalaufnahme	        	10	10	10	10	10	10		Messzeit für eine Signalaufnahme
//! Signalabstand Probenstrom            1	1	1	1	1	1		alle wieviel Sekunden wird ein Signal innerhalb der Signalaufzeichnung genommen
//! Anzahl Messsignale pro Messung		1	1	1	1	1	1		Anzahl Signalaufzeichnungen für eine Messwertbestimmung
//!
//!
//!
//! Service – Parameter
//!
//! Grenzwert relative Feuchte	[%]
//! Einstellpunkt Trägergasdruck	[mbar]	1200
//! Abweichung Trägergasdruck	[%]	10
//! Einstellpunkt Trägergasdurchfluss	[l/h]	5
//! Abweichung Trägergasdurchfluss	[%]	20
//!
//! Parameter Einzelmessung		Bereich	default
//!
//! Füllzeit Einzelmessung		60-1800	600
//! Messwiederholungen		1 – 10	5
//! Ausreisser		0-3	0
//! cV [%]		0-10	2
//!
//! restliche Parameter, wie ausgewählter Probenstrom
//!
//! Serviceparameter	default
//!
//! Volumenstrom Trägergas [l/h]	10		kann manuell gesetzt werden oder durch Kalibrierung automatisch
//! prozentuale Abweichung [%]	20		wird genutzt um Fehler bei Über- oder Unterschreiten zu generieren
//!
//! relative Feuchte [%]	50		Alarmwert für Feuchtefehler
//!
//! Druck (optional) [mbar]	0 (aus)		bei Wert 0 findet keine Auswertung statt
//! prozentual Abweichung [%]	0

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    pub sample_pump:       bool,
    pub sample_valve:      [bool;6],
    pub calibration_valve: bool,
    pub bypass_valve:      bool,
    pub zeroflow_valve:    bool,
    pub tic_valve:         bool,
    pub nullflow_valve:    bool,
    pub ndir1:             f64,
    pub ndir2:             f64,
    pub fluid:             [bool;6],
}


pub trait Uv {
    type Error;
    fn lamp(&mut self,on:bool) -> Result<(),Self::Error>;
    fn sample_pump(&mut self,start:bool) ->  Result<(), Self::Error>;
    fn open_sample_valve(&mut self,sample: u8) ->  Result<(), Self::Error>;
    fn close_sample_valve(&mut self,sample: u8) ->  Result<(), Self::Error>;
    fn open_calibration_valve(&mut self) ->  Result<(), Self::Error>;
    fn close_calibration_valve(&mut self) ->  Result<(), Self::Error>;
    fn open_bypass(&mut self,open:bool)  -> Result<(), Self::Error>;
    fn zeroflow(&mut self,open:bool) -> Result<(), Self::Error>;
    fn tic(&mut self, open: bool) -> Result<(), Self::Error>;
    fn fluid(&self,sample:u8) ->  Result<bool, Self::Error>;
}



#[cfg(feature = "mosk")]
pub mod mosk {
    use nb;
    use crate::error::MockError;
    use crate::common::Generic;
    // pub struct UvSimulate {
        // pub sp: bool,
        // pub mv: [5;bool],
    // }


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
