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
    pub fn to_meters(&self, value: f64 ) -> f64 {
        match self {
            LengthUnit::Meter => value,
            LengthUnit::Kilometer => value * 1000.0,
            LengthUnit::Mile => value * 1609.34,
            LengthUnit::Foot => value * 0.3048,
        }
    }
    pub fn to_kilometers(&self, value: f64) -> f64 {
        match self {
            LengthUnit::Meter => value * (1.0 / 1000.0),
            LengthUnit::Kilometer => 1.0 * value ,
            LengthUnit::Mile => value * (1609.34 / 1000.0),
            LengthUnit::Foot => value * (0.3048 / 1000.0),
        }
    }

    pub fn to_mile(&self, value: f64) -> f64 {
        match self {
            LengthUnit::Kilometer => value * (0.00062137 * 1000.0),
            LengthUnit::Meter => value * 0.00062137,
            LengthUnit::Mile => value * 1.0,
            LengthUnit::Foot => value * 0.00018939
        }
    }
    pub fn to_foot(&self, value: f64) -> f64 {
        match self {
            LengthUnit::Kilometer => value *  3.2808399 * 1000.0,
            LengthUnit::Meter => value * 3.2808399,
            LengthUnit::Mile =>  value * 5280.0,
            LengthUnit::Foot => value * 1.0
        }
    }
  
    pub fn convert(&self, value: &f64, to_unit: LengthUnit) -> String { 
        let result = match to_unit {
            LengthUnit::Kilometer => self.to_kilometers(*value), 
            LengthUnit::Meter => self.to_meters(*value),
            LengthUnit::Mile => self.to_mile(*value),
            LengthUnit::Foot => self.to_foot(*value) 
       };

       format!("{} {}", result, to_unit.short_name())
    }
 
 }
