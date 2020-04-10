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

//! This module provides a structure that keeps some basic info about a line
//! logged by `ps`.
use std::convert::TryFrom;

use regex::Regex;

/// This structure keeps some basic info about an entry of ps.
#[derive(Debug, Clone)]
pub struct PsEntry {
    /// The user id, may be numeric or alphanumeric
    pub uid:  String,
    /// The process' id
    pub pid:  usize,
    /// The parent process' id.
    pub ppid: usize
}

/// The kind of error which is issued when we cannot parse a line.
#[derive(Debug, Clone)]
pub struct ParsingError {
    pub text: String
}

// LINE FORMAT = UID        PID  PPID  C STIME TTY          TIME CMD
const LINE_FMT: &str  = r"^\s*(?P<uid>[^\s]+)\s+(?P<pid>\d+)\s+(?P<ppid>\d+)\s+.*$";
lazy_static! {
    static ref PS_LINE : Regex = Regex::new(LINE_FMT).unwrap();
}
impl TryFrom<&str> for PsEntry {
    type Error = ParsingError;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        if let Some(caps) = PS_LINE.captures(line) {
            Ok(PsEntry {
                uid : caps["uid"].to_string(),
                pid : caps["pid"].parse::<usize>().unwrap(),
                ppid: caps["ppid"].parse::<usize>().unwrap()
            })
        } else {
            Err(ParsingError{text: line.to_string()})
        }
    }
}