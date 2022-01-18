use std::clone::Clone;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
pub enum Notetime {
    Tie = 0b00,
    Normal = 0b01,
    TieRest = 0b10,
    Rest = 0b11,
}

impl FromStr for Notetime {
    type Err = ();
    fn from_str(input: &str) -> Result<Notetime, Self::Err> {
        match input {
            "" => Ok(Notetime::Normal),
            "TI" => Ok(Notetime::Tie),
            "RE" => Ok(Notetime::Rest),
            "TR" => Ok(Notetime::TieRest),
            _ => Err(()),
        }
    }
}

impl Debug for Notetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Notetime::Normal => write!(f, "  "),
            Notetime::Tie => write!(f, "TI"),
            Notetime::Rest => write!(f, "RE"),
            Notetime::TieRest => write!(f, "TR"),
        }
    }
}

impl TryFrom<u8> for Notetime {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Notetime::Normal as u8 => Ok(Notetime::Normal),
            x if x == Notetime::Tie as u8 => Ok(Notetime::Tie),
            x if x == Notetime::Rest as u8 => Ok(Notetime::Rest),
            x if x == Notetime::TieRest as u8 => Ok(Notetime::TieRest),
            _ => Err(()),
        }
    }
}

impl TryFrom<u16> for Notetime {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == Notetime::Normal as u16 => Ok(Notetime::Normal),
            x if x == Notetime::Tie as u16 => Ok(Notetime::Tie),
            x if x == Notetime::Rest as u16 => Ok(Notetime::Rest),
            x if x == Notetime::TieRest as u16 => Ok(Notetime::TieRest),
            _ => Err(()),
        }
    }
}
