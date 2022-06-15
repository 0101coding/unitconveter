mod error;
mod length;
mod unit_kind;
mod mass;
mod temperature;

use std::{io::{BufRead, stdout, stdin, Write}};
use anyhow::{Result, Ok}; 
use error::AppError;
use length::LengthUnit;
use enum_iterator::{all, Sequence};
use unit_kind::{UnitKind, Unit};

#[derive(Debug)]
pub struct ConvertRequest {
    number: f64,
    from_unit: String, 
    to_unit: String
}

impl ConvertRequest {
    pub fn from_text(text: &String) -> Result<Self> {
        let reqs: Vec<&str> = text.split(' ').collect();
        match  reqs.len() {
           1 => {
               // Assum only the from units was supplied
                let (number, from_unit) = parse_from_value(reqs[0].to_owned())?;
                let to_unit  = ask_user("To:");
                return Ok(Self{
                    number,
                    from_unit,
                    to_unit
                });
        
           },
           2 => {
                let (number, from_unit) = parse_from_value(reqs[0].to_owned())?;
                let to_unit  = reqs[1].to_owned();
                return Ok(Self{
                    number,
                    from_unit,
                    to_unit
                });
           },
           3 => {
                if reqs[1] != "to" {
                    return Err(AppError::InvalidFormat(text.into()).into());
                }
                let (number, from_unit) = parse_from_value(reqs[0].to_owned())?;
                let to_unit  = reqs[2].to_owned();
                return Ok(Self{
                    number,
                    from_unit,
                    to_unit
                });
           },
           _ => {
               return Err(AppError::InvalidFormat("Unkown Message".into()).into());
            }
        } 
    }
}

fn ask_user(prompt: &str) -> String {
    let mut line = String::new();
    print!("{} ", prompt);
    let _ = stdout().flush();
    let stdin = stdin();
    stdin.lock().read_line(&mut line).expect("Cannot read from STDIN");
    line.trim_end().to_owned()
}



fn parse_from_value(value: String) -> Result<(f64, String)> {

   let last_number = value.rfind(|c:char| matches!(c, '0'..='9'| '.'));
   if last_number.is_none() {
     return Err(AppError::MissingNumber(value.to_owned()).into());
   }
   let last_number = last_number.unwrap();
   let txt_number = &value[..=last_number];
   let txt_unit = &value[last_number+1..];
   let number: f64 = txt_number.parse::<f64>()?;

   if txt_unit.trim() == "" {
       return Err(AppError::MissingUnit(value.to_owned()).into());
   }
   Ok((number, txt_unit.to_owned()))
}

fn convert(request: ConvertRequest) -> Result<String>{
    let unit = UnitKind::from_text(request.from_unit.as_str())?;
    unit.to_dest_unit(&request.number, &request.to_unit) 
}

fn main() -> Result<()> {
   let args: Vec<String> = std::env::args().skip(1).collect();
  
   let request = match &args.is_empty() {
       true => ask_user("Convert:"),
       false => args.join(" ")
   };
 
   let result = convert(ConvertRequest::from_text(&request.to_lowercase())?)?;

   dbg!(&result);
   Ok(())
}
