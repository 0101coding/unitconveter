use crate::{Unit, UnitKind}; //use crate to bring anything in main
use enum_iterator::Sequence;
use anyhow::Result;
use crate::error::AppError;

#[derive(Debug, Sequence, PartialEq, Eq)]
pub enum LengthUnit {
    Kilometer,
    Meter,
    Mile,
    Foot,
}

impl Unit for LengthUnit {
    fn from_text(t: &str) -> Result<Box<Self>> {
        match UnitKind::from_text(t)? {
            UnitKind::Length(x) => Ok(Box::new(x)),
            _ => Err(AppError::WrongUnit(t.into()).into())
        }
    }
    

    fn short_name(&self) -> &str {
         match self {
            LengthUnit::Meter => "m",
            LengthUnit::Kilometer => "km",
            LengthUnit::Mile => "ml",
            LengthUnit::Foot => "ft",
        }
    }

    fn names(&self) -> Vec<&str> {
        match self {
            LengthUnit::Kilometer => vec!["kilometers", "kilometer", "km"],
            LengthUnit::Meter => vec!["meters", "meter", "m", "mts"],
            LengthUnit::Mile => vec!["ml", "mile", "miles"],
            LengthUnit::Foot => vec!["ft", "foot", "feet"]
        }
    }
}
impl LengthUnit {
    pub fn kilometers(&self) -> f64 {
        match self {
            LengthUnit::Meter => 1000.0,
            LengthUnit::Kilometer => 1.0 ,
            LengthUnit::Mile => 0.62137119,
            LengthUnit::Foot => 3280.8399,
        }
    }
 
  
    pub fn convert(&self, value: &f64, to_unit: LengthUnit) -> String { 
        let result = value * to_unit.kilometers() / self.kilometers(); 

       format!("{}{}", result, to_unit.short_name())
    }
 
 }
