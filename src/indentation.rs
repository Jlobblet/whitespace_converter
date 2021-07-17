use std::str::FromStr;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
pub(crate) enum Indentation {
    Tabs(usize),
    Spaces(usize),
}

impl FromStr for Indentation {
    type Err = anyhow::Error;

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
    pub(crate) fn make_transformation(&self, buf: String) -> String {
        return if let Some(index) = match self {
            Indentation::Tabs(_) => buf.find(|c| c != ' '),
            Indentation::Spaces(_) => buf.find(|c| c != '\t'),
        } {
            if index == 0 {
                return buf;
            }
            let (char, width) = match self {
                Indentation::Tabs(u) => ('\t', index / u),
                Indentation::Spaces(u) => (' ', index * u),
            };
            let mut new = String::with_capacity(buf.capacity());
            new.extend(std::iter::repeat(char).take(width));
            new.reserve_exact(buf.capacity());
            new.push_str(&buf[index..]);
            new
        } else {
            buf
        };
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
        ];
        for (style, input, expected) in cases {
            let actual = style.make_transformation(String::from(input));
            assert_eq!(actual.as_str(), expected);
        }
    }
}
