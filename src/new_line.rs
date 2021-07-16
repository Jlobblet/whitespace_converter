use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
pub(crate) enum NewLine {
    LF,
    CRLF,
}

impl FromStr for NewLine {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match &s.to_ascii_lowercase()[..] {
            "lf" => Ok(NewLine::LF),
            "crlf" => Ok(NewLine::CRLF),
            _ => Err(anyhow!("Unrecognised newline: {}", s)),
        }
    }
}

impl NewLine {
    pub(crate) fn make_transformation(&self, buf: &String) -> String {
        match self {
            NewLine::LF => buf.replace("\r\n", "\n"),
            NewLine::CRLF => buf.replace("\n", "\r\n"),
        }
    }
}
