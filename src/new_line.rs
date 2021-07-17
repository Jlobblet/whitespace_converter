use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug, PartialEq)]
pub(crate) enum NewLine {
    /// Convert newlines to LF (`\n`)
    LF,
    /// Convert newlines to CRLF (`\r\n`)
    CRLF,
}

impl FromStr for NewLine {
    type Err = Error;

    /// Parse a `str` to a `NewLine`.
    ///
    /// Convert to ascii lowercase and then turn `"lf"` into `NewLine::LF`, `"crlf"` into `NewLine::CRLF`.
    ///
    /// ```
    /// let lf = NewLine.from_str("LF").unwrap();
    /// assert_eq!(lf, NewLine::LF);
    ///
    /// let crlf = NewLine.from_str("CRLF").unwrap();
    /// assert_eq!(crlf, NewLine::CRLF);
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "lf" => Ok(NewLine::LF),
            "crlf" => Ok(NewLine::CRLF),
            _ => Err(anyhow!("Unrecognised newline: {}", s)),
        }
    }
}

impl NewLine {
    /// Convert newlines according to what `self` is:
    ///
    /// `NewLine::LF` -> convert CRLF (`\r\n`) to LF (`\n`)
    ///
    /// `NewLine::CRLF` -> convert LF to CRLF
    pub(crate) fn make_transformation(&self, buf: String) -> String {
        match self {
            NewLine::LF if buf.ends_with("\r\n") => buf.replace("\r\n", "\n"),
            NewLine::CRLF if !buf.ends_with("\r\n") => buf.replace("\n", "\r\n"),
            _ => buf,
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
        let cases = vec![
            (NewLine::LF, "Lorem ipsum\r\n", "Lorem ipsum\n"),
            (NewLine::CRLF, "Lorem ipsum\n", "Lorem ipsum\r\n"),
            (NewLine::LF, "Lorem ipsum\n", "Lorem ipsum\n"),
            (NewLine::CRLF, "Lorem ipsum\r\n", "Lorem ipsum\r\n"),
            (NewLine::LF, "Lorem ipsum", "Lorem ipsum"),
            (NewLine::CRLF, "Lorem ipsum", "Lorem ipsum"),
        ];
        for (newline, input, expected) in cases {
            let actual = newline.make_transformation(String::from(input));
            assert_eq!(actual.as_str(), expected);
        }
    }
}
