pub mod length;
pub mod mass;
pub mod temperature;
pub mod time;
 

use anyhow::{Result, Ok};  
use enum_iterator::{all, Sequence};
use crate::error::AppError; 
use crate::ConvertRequest;
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

    fn same_kind(&self, other: &UnitKind) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    } 

    pub fn names(&self) -> Vec<&str> {
        match self {
            UnitKind::Length(x) => x.names(),
            UnitKind::Mass(x) => x.names(),
            UnitKind::Temperature(x) => x.names(),
            UnitKind::Time(x) => x.names()
        }
    }

    pub fn short_name(&self) -> &str {
        match self {
            UnitKind::Length(x) => x.short_name(),
            UnitKind::Mass(x) => x.short_name(),
            UnitKind::Temperature(x) => x.short_name(),
            UnitKind::Time(x) => x.short_name()
        }
    }
   
    pub fn base_conversion(&self, value: Option<f64>) -> f64 {
        match self {
            UnitKind::Length(x) => x.kilometers(),
            UnitKind::Mass(x) => x.kilograms(),
            UnitKind::Temperature(x) => x.from_kelvin(&value.unwrap()),
            UnitKind::Time(x) => x.millenium(),
        }
    }

    pub fn convert(request: &ConvertRequest) -> Result<String>{
        let from_unit = UnitKind::from_text(&request.from_unit)?;
        let to_unit = UnitKind::from_text(&request.to_unit)?;
        println!("{:?}", from_unit);
        if !from_unit.same_kind(&to_unit) {
            return Err(AppError::WrongUnit("You cannot convert from source to destination unit".into()).into());
        }
        let result = match from_unit {
            UnitKind::Temperature(x) => to_unit.base_conversion(Some(x.to_kelvin(&request.number))),
            _ => request.number * to_unit.base_conversion(None) / from_unit.base_conversion(None)
        }; 

        Ok(format!("{}{}", result, to_unit.short_name()))
    }
}