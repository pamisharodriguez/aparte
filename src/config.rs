/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use termion::color;

use crate::account::ConnectionInfo;
use crate::color::ColorTuple;

fn true_() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub accounts: HashMap<String, ConnectionInfo>,
    #[serde(default = "true_")]
    pub bell: bool,
    pub theme: Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub title_bar: ColorTuple,
    pub win_bar: ColorTuple,
    pub roster: ColorTuple,
    pub occupants: ColorTuple,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            title_bar: ColorTuple::new(color::Blue, color::Black),
            win_bar: ColorTuple::new(color::Blue, color::Black),
            roster: ColorTuple::new(color::Blue, color::Black),
            occupants: ColorTuple::new(color::Blue, color::Black),
        }
    }
}
