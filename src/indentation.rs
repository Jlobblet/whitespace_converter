use std::str::FromStr;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
pub(crate) enum Indentation {
    /// Convert indentation to tabs where n spaces becomes one tab
    Tabs(usize),
    /// Convert indentation to spaces where each tab becomes n spaces
    Spaces(usize),
}

impl FromStr for Indentation {
    type Err = anyhow::Error;

    /// Parse a `str` to an `Indentation`.
    /// Expected inputs: `tabs=4` `spaces=4`.
    ///
    /// Convert to ASCII lowercase and then split on `=`.
    /// The left part should be either "tabs" or "spaces",
    /// and the right part is parsed to `usize`.
    fn from_str(s: &str) -> Result<Self> {
        let lower = s.to_ascii_lowercase();
        let (mode, n) = lower.split_once("=").ok_or(anyhow!("fuck"))?;
        let n = usize::from_str(n)?;
        match mode {
            "tabs" => Ok(Indentation::Tabs(n)),
            "spaces" => Ok(Indentation::Spaces(n)),
            _ => Err(anyhow!("Invalid indentation format: {}", s)),
        }
    }
}

impl Indentation {
    /// Convert leading whitespace based on what `self` is.
    /// The wrapped `usize` is how many spaces equal one tab.
    ///
    /// If `self` is `Indentation::Tabs` then convert each n spaces to a tab.
    ///
    /// If `self` is `Indentation::Spaces` then convert each tab to n spaces.
    pub(crate) fn make_transformation(&self, buf: String) -> String {
        let mut tabs: usize = 0;
        let mut spaces: usize = 0;
        // Index at which the rest of the string begins
        let mut index: usize = 0;
        for c in buf.chars() {
            match c {
                '\t' => {
                    tabs += 1;
                    index += 1;
                }
                ' ' => {
                    spaces += 1;
                    index += 1;
                }
                _ => break,
            }
        }
        // There was no indentation at all - return what we were given
        if tabs == 0 && spaces == 0 {
            return buf;
        }
        let (char, count) = match self {
            Indentation::Tabs(u) => ('\t', tabs + spaces / u),
            Indentation::Spaces(u) => (' ', spaces + tabs * u),
        };
        let mut new = String::with_capacity(count + buf.len() - index);
        new.extend(std::iter::repeat(char).take(count));
        new.push_str(&buf[index..]);
        new
    }
}

#[cfg(test)]
mod tests {
    use crate::indentation::Indentation;
    use std::str::FromStr;

    #[test]
    fn test_indentation_from_str() {
        let cases = vec![
            ("TABS=4", Indentation::Tabs(4)),
            ("tabs=4", Indentation::Tabs(4)),
            ("SPACES=4", Indentation::Spaces(4)),
            ("spaces=4", Indentation::Spaces(4)),
        ];
        for (input, expected) in cases {
            let actual = Indentation::from_str(input).unwrap();
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_make_indentation_transformation() {
        let cases = vec![
            (Indentation::Tabs(4), "    Lorem ipsum", "\tLorem ipsum"),
            (Indentation::Tabs(2), "    Lorem ipsum", "\t\tLorem ipsum"),
            (Indentation::Spaces(4), "\tLorem ipsum", "    Lorem ipsum"),
            (Indentation::Spaces(2), "\t\tLorem ipsum", "    Lorem ipsum"),
            (Indentation::Tabs(4), "\tLorem ipsum", "\tLorem ipsum"),
            (Indentation::Spaces(4), "    Lorem ipsum", "    Lorem ipsum"),
            (Indentation::Tabs(2), "\t  Lorem ipsum", "\t\tLorem ipsum"),
            (Indentation::Spaces(2), "  \tLorem ipsum", "    Lorem ipsum"),
        ];
        for (style, input, expected) in cases {
            let actual = style.make_transformation(String::from(input));
            assert_eq!(actual.as_str(), expected);
        }
    }
}
