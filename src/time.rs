use std::clone::Clone;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
pub enum Time {
    Tie = 0b00,
    Normal = 0b01,
    TieRest = 0b10,
    Rest = 0b11,
}

impl FromStr for Time {
    type Err = ();
    fn from_str(input: &str) -> Result<Time, Self::Err> {
        match input {
            "" => Ok(Time::Normal),
            "TI" => Ok(Time::Tie),
            "RE" => Ok(Time::Rest),
            "TR" => Ok(Time::TieRest),
            _ => Err(()),
        }
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Time::Normal => write!(f, "  "),
            Time::Tie => write!(f, "TI"),
            Time::Rest => write!(f, "RE"),
            Time::TieRest => write!(f, "TR"),
        }
    }
}

impl TryFrom<u8> for Time {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Time::Normal as u8 => Ok(Time::Normal),
            x if x == Time::Tie as u8 => Ok(Time::Tie),
            x if x == Time::Rest as u8 => Ok(Time::Rest),
            x if x == Time::TieRest as u8 => Ok(Time::TieRest),
            _ => Err(()),
        }
    }
}

impl TryFrom<u16> for Time {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == Time::Normal as u16 => Ok(Time::Normal),
            x if x == Time::Tie as u16 => Ok(Time::Tie),
            x if x == Time::Rest as u16 => Ok(Time::Rest),
            x if x == Time::TieRest as u16 => Ok(Time::TieRest),
            _ => Err(()),
        }
    }
}
