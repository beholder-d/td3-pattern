use std::clone::Clone;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
pub enum Time {
    Normal,
    Tie,
    Rest,
}

impl FromStr for Time {
    type Err = ();
    fn from_str(input: &str) -> Result<Time, Self::Err> {
        match input {
            "" => Ok(Time::Normal),
            "TI" => Ok(Time::Tie),
            "RS" => Ok(Time::Rest),
            _ => Err(()),
        }
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Time::Normal => write!(f, "  "),
            Time::Tie => write!(f, "TI"),
            Time::Rest => write!(f, "RS"),
        }
    }
}
