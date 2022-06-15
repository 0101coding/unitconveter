use crate::{Unit, UnitKind}; //use crate to bring anything in main
use enum_iterator::Sequence;
use anyhow::Result;
use crate::error::AppError;

#[derive(Debug, Sequence, PartialEq, Eq)]
pub enum MassUnit {
    Kilogram,
    Gram,
    Pound,
    Ounce
}

impl Unit for MassUnit {
     fn from_text(t: &str) -> Result<Box<Self>> {
        match UnitKind::from_text(t)? {
            UnitKind::Mass(x) => Ok(Box::new(x)),
            _ => Err(AppError::WrongUnit(t.into()).into())
        }
    }

    fn short_name(&self) -> &str {
         match self {
            MassUnit::Gram => "g",
            MassUnit::Kilogram => "kg",
            MassUnit::Pound => "lb",
            MassUnit::Ounce => "oz"
        }
    }
     

    fn names(&self) -> Vec<&str> {
        match self {
            MassUnit::Gram  => vec!["g", "gram", "grams"],
            MassUnit::Kilogram => vec!["kg", "kilogram", "kilograms"],
            MassUnit::Pound => vec!["lb", "lbs", "pound", "pounds"],
            MassUnit::Ounce => vec!["oz", "ounce", "ounces"]
        }
    }

}


impl MassUnit {
    pub fn to_grams(&self, value:f64) -> f64 {
        match self {
            MassUnit::Kilogram => 1000.0,
            MassUnit::Gram => 1.0,
            MassUnit::Pound => 453.59237,
            MassUnit::Ounce => 28.3495231,
        }
    }
    pub fn to_kilograms(&self, value:f64) -> f64 {
        match self {
            MassUnit::Kilogram => 1.0,
            MassUnit::Gram => 1.0 / 1000.0,
            MassUnit::Pound => 0.45359237,
            MassUnit::Ounce => 0.02834952,
        }
    }
    pub fn to_ounces(&self, value:f64) -> f64 {
        match self {
            MassUnit::Kilogram => 35.273962,
            MassUnit::Gram => 0.03527396,
            MassUnit::Pound => 16.0,
            MassUnit::Ounce => 1.0,
        }
    }
    pub fn to_pounds(&self, value: f64) -> f64 {
        match self {
            MassUnit::Kilogram => 2.20462262,
            MassUnit::Gram => 0.00220462,
            MassUnit::Pound => 1.0,
            MassUnit::Ounce => 0.0625,
        }
    }
    pub fn convert(&self, value: &f64, to_unit: MassUnit) -> String {
        let result = match to_unit {
            MassUnit::Kilogram => self.to_kilograms(*value),
            MassUnit::Gram => self.to_grams(*value),
            MassUnit::Pound => self.to_pounds(*value),
            MassUnit::Ounce => self.to_ounces(*value),
        };

       format!("{} {}", result, to_unit.short_name())
    }
}