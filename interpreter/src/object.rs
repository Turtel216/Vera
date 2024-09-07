// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

#[derive(PartialEq)]
pub struct ObjString {
    pub chars: String,
}

use std::fmt;

impl fmt::Display for ObjString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.chars)
    }
}

impl Clone for ObjString {
    fn clone(&self) -> Self {
        ObjString {
            chars: self.chars.clone(),
        }
    }
}
