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

//! This module provides the an Error type for graceful handling of everything
//! what can go wrong in my code.

use crate::psentry::ParsingError;
use std::num::ParseIntError;

/// Similarly to what is done in the stdlib io module, I declare my own Result
/// alias that can either hold a success value, or hold the error that occurred.
pub type Result<T> = std::result::Result<T, Error>;

/// This enum is basically an umbrella error type that will just bubble up the
/// error that might have arisen while processing the command.
#[derive(Debug)]
pub enum Error {
    Utf8Error   {e: std::string::FromUtf8Error},
    IOError     {e: std::io::Error},
    RegexError  {e: regex::Error},
    ParsingError{e: ParsingError},
    ParseIntErr {e: ParseIntError}
}
impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Error::ParseIntErr {e}
    }
}
impl From<ParsingError> for Error {
    fn from(e: ParsingError) -> Self {
        Error::ParsingError {e}
    }
}
impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error::Utf8Error {e}
    }
}
impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Error::RegexError {e}
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError {e}
    }
}
