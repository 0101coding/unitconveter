use crate::{Unit, UnitKind}; //use crate to bring anything in main
use enum_iterator::Sequence;
use anyhow::Result;
use crate::error::AppError;

#[derive(Debug, Sequence, PartialEq, Eq)]
pub enum TemperatureUnit {
     Celcius,
     Fahrenheit,
     Kelvin
}

impl Unit for TemperatureUnit {
    fn from_text(t: &str) -> Result<Box<Self>> {
        match UnitKind::from_text(t)? {
            UnitKind::Temperature(x) => Ok(Box::new(x)),
            _ => Err(AppError::WrongUnit(t.into()).into())
        }
    }

    fn short_name(&self) -> &str {
        match self {
            TemperatureUnit::Celcius => "C" ,
            TemperatureUnit::Fahrenheit => "F",
            TemperatureUnit::Kelvin => "K",
        }
    }
    fn names(&self) -> Vec<&str> {
        match  self  {
            TemperatureUnit::Celcius => vec!["c", "cel", "celcius"] ,
            TemperatureUnit::Fahrenheit => vec!["f", "farenheit", "fahrenheit"],
            TemperatureUnit::Kelvin => vec!["k", "kelvin"],
        }
    }
}

impl TemperatureUnit {
   pub fn to_celcius(&self, value: f64) -> f64 {
       match  self {
        TemperatureUnit::Celcius => 1.0,
        TemperatureUnit::Fahrenheit => (value - 32.0) * (5.0/9.0),
        TemperatureUnit::Kelvin => value - 273.15,
    }
   } 

   pub fn to_kelvin(&self, value: f64) -> f64 {
       match  self {
        TemperatureUnit::Celcius => value + 273.15,
        TemperatureUnit::Fahrenheit => (value - 32.0) * (5.0/9.0) + 273.15,
        TemperatureUnit::Kelvin => value,
    }
   }
   
   pub fn to_farenheit(&self, value: f64) -> f64 {
       match self {
        TemperatureUnit::Celcius => (value * 9.5) + 32.0 ,
        TemperatureUnit::Fahrenheit => value,
        TemperatureUnit::Kelvin => (value - 273.15) * (9.0/5.0) + 32.0,
    }
   }

   pub fn convert(&self, value: &f64, to_unit: TemperatureUnit) -> String {
     let result  = match to_unit{
        TemperatureUnit::Celcius => self.to_celcius(*value),
        TemperatureUnit::Fahrenheit => self.to_farenheit(*value),
        TemperatureUnit::Kelvin => self.to_kelvin(*value),
    };

    format!("{} {}", result, to_unit.short_name())
 }

}