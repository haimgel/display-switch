//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use anyhow::Result;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum SymbolicInputSource {
    DisplayPort1 = 0x0f,
    DisplayPort2 = 0x10,
    Hdmi1 = 0x11,
    Hdmi2 = 0x12,
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

    pub fn value(self) -> u16 {
        match self {
            Self::Symbolic(sym) => sym as u16,
            Self::Raw(value) => value,
        }
    }
}
