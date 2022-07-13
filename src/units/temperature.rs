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
            TemperatureUnit::Celcius => "°C" ,
            TemperatureUnit::Fahrenheit => "°F",
            TemperatureUnit::Kelvin => "°K",
        }
    }
    fn names(&self) -> Vec<&str> {
        match  self  {
            TemperatureUnit::Celcius => vec!["°c", "cel", "celcius"] ,
            TemperatureUnit::Fahrenheit => vec!["°f", "farenheit", "fahrenheit"],
            TemperatureUnit::Kelvin => vec!["°k", "k", "kelvin"],
        }
    }
}

impl TemperatureUnit {

   pub fn to_kelvin(&self, value: &f64) -> f64 {
       match  self {
        TemperatureUnit::Celcius => value + 273.15,
        TemperatureUnit::Fahrenheit => (value - 32.0) * (5.0/9.0) + 273.15,
        TemperatureUnit::Kelvin => *value,
    }
   }

   pub fn from_kelvin(&self, value: &f64) -> f64 {
        match self {
            TemperatureUnit::Celcius => value - 273.15,
            TemperatureUnit::Fahrenheit => (value - 273.15) / (5.0/9.0) + 32.0,
            TemperatureUnit::Kelvin => *value,
        }
   }

}