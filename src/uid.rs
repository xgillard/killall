// Copyright 2020 Xavier Gillard
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! This module provides a structure to identify someone. I implemented it
//! because linux and OSX do not display the uid in the same way in ps.
//! OSX shows an uid (number) while linux shows the username. Hence my
//! structure keeps both. (To that end, it queries the command `id`).
use std::process::Command;

use crate::error::Result;

/// A structure to keep the username and uid of someone.
#[derive(Debug, Clone)]
pub struct UID {
    pub uid   : String,
    pub uname : String
}

impl UID {
    /// Returns true iff uid is either the user name or it uid.
    pub fn matches(&self, uid: &str) -> bool {
        uid.eq(&self.uid) || uid.eq(&self.uname)
    }

    /// Returns an uid for a given user
    pub fn get(name: &Option<String>) -> Result<UID> {
        Ok(UID {
            uid   : UID::uid(&name)?,
            uname : UID::uname(&name)?
        })
    }

    fn uid(name: &Option<String>) -> Result<String> {
        let output = if let Some(name) = name {
            Command::new("id").arg("-u").arg(name).output()
        } else {
            Command::new("id").arg("-u").output()
        }?;

        let uid = String::from_utf8(output.stdout)?.trim().to_string();

        Ok(uid)
    }

    fn uname(name: &Option<String>) -> Result<String> {
        let output = if let Some(name) = name {
            Command::new("id").arg("-un").arg(name).output()
        } else {
            Command::new("id").arg("-un").output()
        }?;

        let uname = String::from_utf8(output.stdout)?.trim().to_string();

        Ok(uname)
    }
}