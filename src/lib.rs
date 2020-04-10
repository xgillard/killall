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

//! This module provides the functions I use to implement the killall tool.
//! Namely, `kill`, `list_all`, `list_descendants` and `list_matches`.

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::convert::TryFrom;
use std::process::Command;

use regex::Regex;

pub use error::{Error, Result};
pub use psentry::PsEntry;
pub use uid::UID;

mod error;
mod uid;
mod psentry;

/// Kills the process identified with the given PsEntry
pub fn kill(job: &PsEntry) -> Result<()> {
    let _ = Command::new("kill").arg(format!("{}", job.pid)).output()?;
    Ok(())
}

/// Lists all the processes (note: the bulk of the work is deferred to `ps`).
///
/// # Note
/// Using this function will trigger a call to `ps -e` on your system.
pub fn list_all() -> Result<Vec<PsEntry>> {
    let output = Command::new("ps").arg("e").arg("-o").arg("uid pid ppid command").output()?;
    let output = String::from_utf8(output.stdout)?;

    let entries= output.lines().skip(1) // skip header
        .map(|l| PsEntry::try_from(l).unwrap())
        .collect::<Vec<PsEntry>>();

    Ok(entries)
}

/// Lists all the processes that are directly (or indirectly) children of
/// the process whose pid is given by `parent`.
///
/// # Note
/// Using this function will trigger a call to `ps -alx` on your system.
pub fn list_descendants(parent: usize) -> Result<Vec<PsEntry>> {
    let all = list_all()?;

    let mut ret      = vec![];
    let mut frontier = vec![];

    if let Some(root) = all.iter().find(|e| e.pid == parent) {
        frontier.push(root.clone());

        while !frontier.is_empty() {
            let curr = frontier.pop().unwrap();
            all.iter().filter(|e| e.ppid == curr.pid).for_each(|e| frontier.push(e.clone()));

            ret.push(curr);
        }
    }

    Ok(ret)
}

/// Lists all the processes for which the command matches the given pattern.
///
/// # Note
/// Using this function will trigger a call to `ps -alx` on your system.
pub fn list_matches(pattern: &str) -> Result<Vec<PsEntry>> {
    let pattern= Regex::new(pattern)?;

    let mut entries= list_all()?;
    entries.retain(|j| pattern.is_match(&j.command));

    Ok(entries)
}