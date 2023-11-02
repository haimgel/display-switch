//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use paste::paste;
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::convert::TryFrom;
use std::fmt;

macro_rules! symbolic_input_source {
    (
        $($name:ident: $value:expr)*
    ) => {
        #[derive(Clone, Copy, Debug)]
        pub enum SymbolicInputSource {
            $($name = $value,)*
        }

        impl TryFrom<u16> for SymbolicInputSource {
            type Error = ();

            fn try_from(v: u16) -> Result<Self, Self::Error> {
                match v {
                    $($value => Ok(Self::$name),)*
                    _ => Err(()),
                }
            }
        }

        impl TryFrom<&str> for SymbolicInputSource {
            type Error = ();

            fn try_from(v: &str) -> Result<Self, Self::Error> {
                paste! {
                    match v.to_lowercase().as_str() {
                        $(stringify!([< $name:lower >]) => Ok(Self::$name),)*
                        _ => Err(()),
                    }
                }
            }
        }
    }
}

symbolic_input_source! {
    DisplayPort1: 0x0f
    DisplayPort2: 0x10
    Hdmi1: 0x11
    Hdmi2: 0x12
    CompositeVideo1: 0x05
    CompositeVideo2: 0x06
    Dvi1: 0x3
    Dvi2: 0x4
    Vga1: 0x1
}

#[derive(Clone, Copy)]
pub enum InputSource {
    Raw(u16),
    Symbolic(SymbolicInputSource),
}

impl InputSource {
    pub fn value(&self) -> u16 {
        match self {
            Self::Symbolic(sym) => *sym as u16,
            Self::Raw(value) => *value,
        }
    }

    pub fn normalize(self) -> Self {
        match self {
            Self::Symbolic(_) => self,
            Self::Raw(value) => SymbolicInputSource::try_from(value)
                .map(Self::Symbolic)
                .unwrap_or(Self::Raw(value)),
        }
    }
}

fn parse_int(s: &str) -> Result<u16, std::num::ParseIntError> {
    if s.starts_with("0x") {
        // Parse as hexadecimal
        u16::from_str_radix(&s[2..], 16)
    } else {
        // Parse as decimal
        s.parse::<u16>()
    }
}

impl<'de> Deserialize<'de> for InputSource {
    fn deserialize<D>(deserializer: D) -> Result<InputSource, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?.trim().to_lowercase();
        if let Ok(val) = parse_int(&str) {
            Ok(Self::Raw(val).normalize())
        }else {
            SymbolicInputSource::try_from(str.as_str())
                .map(Self::Symbolic)
                .map_err(|_| D::Error::custom(format!("Invalid input source: {}", str)))
        }
    }
}

impl From<InputSource> for u16 {
    fn from(val: InputSource) -> Self {
        val.value()
    }
}

impl From<u16> for InputSource {
    fn from(value: u16) -> Self {
        Self::Raw(value).normalize()
    }
}

impl fmt::Display for SymbolicInputSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for InputSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Symbolic(sym) => write!(f, "{}(0x{:x})", sym, *sym as u16),
            Self::Raw(value) => write!(f, "Custom(0x{:x})", value),
        }
    }
}

impl fmt::Debug for InputSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}
