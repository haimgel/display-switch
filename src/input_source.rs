//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::convert::TryFrom;
use std::fmt;

macro_rules! symbolic_input_source {
    (
        $($name:ident: $value:expr)*
    ) => {
        #[derive(Clone, Copy, Debug, Deserialize)]
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
                match v.to_lowercase().as_str() {
                    $(stringify!(lower!($name)) => Ok(Self::$name),)*
                    _ => Err(()),
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

impl<'de> Deserialize<'de> for InputSource {
    fn deserialize<D>(deserializer: D) -> Result<InputSource, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?.trim().to_lowercase();
        let val = SymbolicInputSource::try_from(str.as_str());
        if let Ok(sym) = val {
            Ok(Self::Symbolic(sym))
        } else {
            let result;
            if str.starts_with("0x") {
                result = u16::from_str_radix(str.trim_start_matches("0x"), 16);
            } else {
                result = u16::from_str_radix(&str, 10);
            }
            result
                .map(|val| Self::Raw(val).normalize())
                .map_err(|err| D::Error::custom(format!("{:?}", err)))
        }
    }
}

impl Into<u16> for InputSource {
    fn into(self) -> u16 {
        self.value()
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
