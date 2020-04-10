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

//! This is my take on what I wish killall was (and I needed because I could not
//! find the sources of killall to install it on a system where I needed it).

extern crate structopt;
extern crate killall;

use structopt::StructOpt;
use killall::{Result, list_matches, UID, PsEntry, list_descendants, kill};

/// Killall Lets you either kill all processes matching a given regex
/// (optionally filtering if the process belongs to some predefined user).
///
/// Or it can kill a whole tree of processes descending from some root process pid.
/// (again with the possibility of filtering on the owner of the processes).
///
/// The default owner for all process is always the current user.
#[derive(StructOpt)]
enum Args {
    /// This subcommand lets you kill all the processes that match a given
    /// pattern. Note: Because killall is basically a shim around ps, you
    /// might need to be a little bit careful with your 'name'. Indeed, the
    /// filtering is done as if it were done by `ps -alx | grep $pattern`.
    Matching{
        /// This is the name (or a pattern) to identify the processes to kill
        name: String,
        /// If set, only the processes belonging to this user will be killed
        belonging_to: Option<String>
    },
    /// This subcommand lets you kill all the processes that were spawned by
    /// the `pid` process, or any of its descendants.
    ChildrenOf {
        /// The pid of the process that spawned (directly or not) all the other
        /// processes we wish to kill.
        pid : usize,
        /// If set, only the processes belonging to this user will be killed
        belonging_to: Option<String>
    },
}

impl Args {
    fn jobs(self) -> Result<Vec<PsEntry>> {
        match self {
            Args::Matching {name, belonging_to} => {
                let owner= UID::get(&belonging_to)?;
                let jobs = list_matches(&name)?.drain(..)
                    .filter(|j| owner.matches(&j.uid))
                    .collect::<Vec<PsEntry>>();

                Ok(jobs)
            },
            Args::ChildrenOf{pid,  belonging_to} => {
                let owner = UID::get(&belonging_to)?;
                let jobs  = list_descendants(pid)?.drain(..)
                    .filter(|j| owner.matches(&j.uid))
                    .collect::<Vec<PsEntry>>();

                Ok(jobs)
            }
        }
    }
}

fn main() -> Result<()>{
    let jobs = Args::from_args().jobs()?;

    for job in jobs.iter() {
        kill(job)?;
    }

    Ok(())
}
