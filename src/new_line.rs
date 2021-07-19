use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug, PartialEq)]
pub(crate) enum NewLine {
    /// Convert newlines to LF (`\n`)
    Lf,
    /// Convert newlines to CRLF (`\r\n`)
    Crlf,
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
            "lf" => Ok(NewLine::Lf),
            "crlf" => Ok(NewLine::Crlf),
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
            NewLine::Lf if buf.ends_with("\r\n") => buf.replace("\r\n", "\n"),
            NewLine::Crlf if !buf.ends_with("\r\n") => buf.replace("\n", "\r\n"),
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
            ("LF", NewLine::Lf),
            ("lf", NewLine::Lf),
            ("CRLF", NewLine::Crlf),
            ("crlf", NewLine::Crlf),
        ];
        for (input, expected) in cases {
            let actual = NewLine::from_str(input).unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_make_newline_transformation() {
        let cases = vec![
            (NewLine::Lf, "Lorem ipsum\r\n", "Lorem ipsum\n"),
            (NewLine::Crlf, "Lorem ipsum\n", "Lorem ipsum\r\n"),
            (NewLine::Lf, "Lorem ipsum\n", "Lorem ipsum\n"),
            (NewLine::Crlf, "Lorem ipsum\r\n", "Lorem ipsum\r\n"),
            (NewLine::Lf, "Lorem ipsum", "Lorem ipsum"),
            (NewLine::Crlf, "Lorem ipsum", "Lorem ipsum"),
        ];
        for (newline, input, expected) in cases {
            let actual = newline.make_transformation(String::from(input));
            assert_eq!(actual.as_str(), expected);
        }
    }
}
