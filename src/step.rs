use std::clone::Clone;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::str::FromStr;

const EMPTY: &'static str = "";

#[derive(Debug, Copy, Clone)]
pub struct Step {
    pub note: u8,
    pub transpose: Transpose,
    pub accent: Accent,
    pub slide: Slide,
    pub time: Time,
}

impl Default for Step {
    fn default() -> Step {
        Step { note: 0, transpose: Transpose::Normal, accent: Accent::Off, slide: Slide::Off, time: Time::Normal }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Transpose {
    Down = 0,
    Normal = 1,
    Up = 2,
}

const UP: &'static str = "UP";
const DN: &'static str = "DN";

impl FromStr for Transpose {
    type Err = ();
    fn from_str(input: &str) -> Result<Transpose, Self::Err> {
        let input = input.to_uppercase();
        let input = input.trim();
        match input {
            DN => Ok(Transpose::Down),
            EMPTY => Ok(Transpose::Normal),
            UP => Ok(Transpose::Up),
            _ => Err(()),
        }
    }
}

impl Debug for Transpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Transpose::Down => write!(f, "{:2?}", UP),
            Transpose::Normal => write!(f, "{:2?}", EMPTY),
            Transpose::Up => write!(f, "{:2?}", DN),
        }
    }
}

impl TryFrom<u8> for Transpose {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Transpose::Down as u8 => Ok(Transpose::Down),
            x if x == Transpose::Normal as u8 => Ok(Transpose::Normal),
            x if x == Transpose::Up as u8 => Ok(Transpose::Up),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Accent {
    Off = 0,
    On = 1,
}

const AC: &'static str = "AC";

impl FromStr for Accent {
    type Err = ();
    fn from_str(input: &str) -> Result<Accent, Self::Err> {
        let input = input.to_uppercase();
        let input = input.trim();
        match input {
            EMPTY => Ok(Accent::Off),
            AC => Ok(Accent::On),
            _ => Err(()),
        }
    }
}

impl Debug for Accent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Accent::Off => write!(f, "{:2?}", EMPTY),
            Accent::On => write!(f, "{:2?}", AC),
        }
    }
}

impl TryFrom<u8> for Accent {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Accent::On as u8 => Ok(Accent::On),
            x if x == Accent::Off as u8 => Ok(Accent::Off),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Slide {
    Off = 0,
    On = 1,
}

const SL: &'static str = "SL";

impl FromStr for Slide {
    type Err = ();
    fn from_str(input: &str) -> Result<Slide, Self::Err> {
        let input = input.to_uppercase();
        let input = input.trim();
        match input {
            EMPTY => Ok(Slide::Off),
            SL => Ok(Slide::On),
            _ => Err(()),
        }
    }
}

impl Debug for Slide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Slide::Off => write!(f, "{:2?}", EMPTY),
            Slide::On => write!(f, "{:2?}", SL),
        }
    }
}

impl TryFrom<u8> for Slide {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == Slide::On as u8 => Ok(Slide::On),
            x if x == Slide::Off as u8 => Ok(Slide::Off),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Time {
    Tie = 0b00,
    Normal = 0b01,
    TieRest = 0b10,
    Rest = 0b11,
}

const TI: &'static str = "TI";
const RE: &'static str = "RE";
const TR: &'static str = "TR";

impl FromStr for Time {
    type Err = ();
    fn from_str(input: &str) -> Result<Time, Self::Err> {
        let input = input.to_uppercase();
        let input = input.trim();
        match input {
            EMPTY => Ok(Time::Normal),
            TI => Ok(Time::Tie),
            RE => Ok(Time::Rest),
            TR => Ok(Time::TieRest),
            _ => Err(()),
        }
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Time::Normal => write!(f, "{:2?}", EMPTY),
            Time::Tie => write!(f, "{:2?}", TI),
            Time::Rest => write!(f, "{:2?}", RE),
            Time::TieRest => write!(f, "{:2?}", TR),
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
