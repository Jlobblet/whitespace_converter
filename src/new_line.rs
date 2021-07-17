use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::new_line::NewLine;
    use std::str::FromStr;

    #[test]
    fn test_newline_from_str() {
        let cases = vec![
            ("LF", NewLine::LF),
            ("lf", NewLine::LF),
            ("CRLF", NewLine::CRLF),
            ("crlf", NewLine::CRLF),
        ];
        for (input, expected) in cases {
            let actual = NewLine::from_str(input).unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_make_newline_transformation() {
        let crlf = String::from("Lorem ipsum\r\n");
        let lf = String::from("Lorem ipsum\n");

        let actual = NewLine::LF.make_transformation(&crlf);
        assert_eq!(actual, lf);

        let actual = NewLine::CRLF.make_transformation(&lf);
        assert_eq!(actual, crlf);
    }
}
