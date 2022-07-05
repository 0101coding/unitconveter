
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Missing number converting from {0:?}")]
    MissingNumber(String),
    #[error("Missing unit while converting {0:?}")]
    MissingUnit(String),
    #[error("Invalid Format - e.g 10kg to g")]
    InvalidFormat(String),
   #[error("This unit ({0}) is not known or not inlcuded yet in the Conversion")] 
    UnknownUnit(String),
    #[error("Cannot parse from Source to Dest Unit")] 
    WrongUnit(String)
}