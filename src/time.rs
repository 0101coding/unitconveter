use std::fmt::format;

use crate::{Unit, UnitKind};
use enum_iterator::Sequence;
use anyhow::{Result, Ok};
use crate::error::AppError;

#[derive(Debug, Sequence, PartialEq, Eq)]
pub enum TimeUnit {
    Millenium, 
    Century,
    Decade,
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

impl Unit for TimeUnit {
    fn from_text(t: &str) -> Result<Box<Self>> {
        match UnitKind::from_text(t)? {
           UnitKind::Time(x) => Ok(Box::new(x)),
           _ =>  Err(AppError::WrongUnit(t.into()).into())
        }
    }

    fn short_name(&self) -> &str {
       match self {
            TimeUnit::Millenium => "M",
            TimeUnit::Century => "cent",
            TimeUnit::Decade => "dec",
            TimeUnit::Year => "yr.",
            TimeUnit::Month => "mo",
            TimeUnit::Week => "wk",
            TimeUnit::Day => "d",
            TimeUnit::Hour => "hr",
            TimeUnit::Minute => "min",
            TimeUnit::Second => "sec" // or s
       } 
    }

    fn names(&self) -> Vec<&str> {
        match self {
            TimeUnit::Millenium => vec!["M", "Millenium"],
            TimeUnit::Century => vec!["cent", "century"],
            TimeUnit::Decade =>  vec!["dec", "decade", "decades"],
            TimeUnit::Year => vec!["yr.", "yrs", "year", "years"],
            TimeUnit::Month => vec!["mo", "month", "months"],
            TimeUnit::Week => vec!["wk", "week","weeks"],
            TimeUnit::Day => vec!["d", "day", "days"],
            TimeUnit::Hour => vec!["hr", "hrs", "hour", "hours"],
            TimeUnit::Minute => vec!["min", "minute", "minutes"],
            TimeUnit::Second => vec!["s", "sec", "second", "seconds"] // or s
       } 
    }

}

impl TimeUnit {
    pub fn millenium(&self) -> f64 {
        match self {
            TimeUnit::Millenium => 0.001,
            TimeUnit::Century => 0.01,
            TimeUnit::Decade => 0.1,
            TimeUnit::Year => 1.0,
            TimeUnit::Month => 12.0,
            TimeUnit::Week => 52.1429,
            TimeUnit::Day => 365.0,
            TimeUnit::Hour => 8760.0,
            TimeUnit::Minute => 525600.0,
            TimeUnit::Second => 31540000.0 // or s
        }
    }

    pub fn convert(&self, value: &f64, to_unit: TimeUnit) -> String {
        let result = value * to_unit.millenium() / self.millenium();

        format!("{:.5}{}", result, to_unit.short_name())
    }
}
