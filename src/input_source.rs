//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::convert::TryFrom;

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
    }
}

symbolic_input_source! {
    DisplayPort1: 0x0f
    DisplayPort2: 0x10
    Hdmi1: 0x11
    Hdmi2: 0x12
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(untagged)]
pub enum InputSource {
    #[serde(deserialize_with = "InputSource::deserialize_raw")]
    Raw(u16),
    Symbolic(SymbolicInputSource),
}

impl InputSource {
    fn deserialize_raw<'de, D>(deserializer: D) -> Result<u16, D::Error>
        where
            D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?.trim().to_lowercase();
        let result;
        if str.starts_with("0x") {
            result = u16::from_str_radix(str.trim_start_matches("0x"), 16);
        } else {
            result = u16::from_str_radix(&str, 10);
        }
        result.map_err(|err| D::Error::custom(format!("{:?}", err)))
    }

    pub fn value(&self) -> u16 {
        match self {
            Self::Symbolic(sym) => *sym as u16,
            Self::Raw(value) => *value,
        }
    }

    pub fn normalize(self) -> Self {
        match self {
            Self::Symbolic(_) => self,
            Self::Raw(value) => {
                SymbolicInputSource::try_from(value)
                    .map(|sym| Self::Symbolic(sym))
                    .unwrap_or(Self::Raw(value))
            }
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
            Self::Symbolic(sym) => write!(f, "{}", sym),
            Self::Raw(value) => write!(f, "Custom(0x{:x})", value),
        }
    }
}
