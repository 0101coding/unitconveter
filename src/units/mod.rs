pub mod length;
pub mod mass;
pub mod temperature;
pub mod time;

use anyhow::{Result, Ok};  
use enum_iterator::{all, Sequence};
use crate::error::AppError; 

use length::LengthUnit;
use mass::MassUnit;
use temperature::TemperatureUnit;
use time::TimeUnit;

pub trait Unit {
    fn from_text(t: &str) -> Result<Box<Self>> ; 
    fn short_name(&self) -> &str;
    fn names(&self) -> Vec<&str>;
}

#[derive(Debug, PartialEq, Eq, Sequence)]
pub enum  UnitKind {
    Length(LengthUnit),
    Mass(MassUnit),
    Temperature(TemperatureUnit),
    Time(TimeUnit)
}

impl UnitKind {
     pub fn from_text(t: &str) -> Result<Self> {
         for unit in all::<UnitKind>() {
           let names = unit.names();
           for name in names {
               if name == t {
                   return Ok(unit);
               }
           } 
         } 
         return Err(AppError::UnknownUnit(t.into()).into())
    }

    pub fn names(&self) -> Vec<&str> {
        match self {
            UnitKind::Length(x) => x.names(),
            UnitKind::Mass(x) => x.names(),
            UnitKind::Temperature(x) => x.names(),
            UnitKind::Time(x) => x.names()
        }
    }
  

    pub fn to_dest_unit(&self, value: &f64, to_unit: &str) -> Result<String> {
        match self {
            UnitKind::Length(ref x) => Ok(x.convert(value, *LengthUnit::from_text(to_unit)?)),
            UnitKind::Mass(x) => Ok(x.convert(value, *MassUnit::from_text(to_unit)?)),
            UnitKind::Temperature(x) => Ok(x.convert(value, *TemperatureUnit::from_text(to_unit)?)),
            UnitKind::Time(x) => Ok(x.convert(value, *TimeUnit::from_text(to_unit)?))
        }
    }
}
