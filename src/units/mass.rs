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
    pub fn kilograms(&self) -> f64 {
        match self {
            MassUnit::Kilogram => 1.0,
            MassUnit::Gram =>  1000.0,
            MassUnit::Pound => 2.20462262,
            MassUnit::Ounce => 35.273962,
        }
    }
    pub fn convert(&self, value: &f64, to_unit: MassUnit) -> String {
        let result = value * to_unit.kilograms() / self.kilograms(); 
        format!("{} {}", result, to_unit.short_name())
    }
}
