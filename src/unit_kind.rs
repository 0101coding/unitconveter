use anyhow::{Result, Ok};
use crate::length::LengthUnit;
use crate::mass::MassUnit;
use enum_iterator::{all, Sequence};
use crate::error::AppError;
use crate::temperature::TemperatureUnit;

pub trait Unit {
    fn from_text(t: &str) -> Result<Box<Self>> ; 
    fn short_name(&self) -> &str;
    fn names(&self) -> Vec<&str>;
}

#[derive(Debug, PartialEq, Eq, Sequence)]
pub enum  UnitKind {
    Length(LengthUnit),
    Mass(MassUnit),
    Temperature(TemperatureUnit)
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
            UnitKind::Temperature(x) => todo!(),
        }
    }
  

    pub fn to_dest_unit(&self, value: &f64, to_unit: &str) -> Result<String> {
        match self {
            UnitKind::Length(ref x) => Ok(x.convert(value, *LengthUnit::from_text(to_unit)?)),
            UnitKind::Mass(x) => Ok(x.convert(value, *MassUnit::from_text(to_unit)?)),
            UnitKind::Temperature(x) => Ok(x.convert(value, *TemperatureUnit::from_text(to_unit)?)),
        }
    }
}
