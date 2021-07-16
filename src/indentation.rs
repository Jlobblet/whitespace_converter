use std::str::FromStr;

use anyhow::{anyhow, Result};

#[derive(Debug)]
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
            Indentation::Spaces(_) => buf.find(|c| c != '\t')
        } {
            if index == 0 {
                return buf;
            }
            let mut new = String::with_capacity(buf.capacity());
            let (char, width) = match self {
                Indentation::Tabs(u) => {
                    ('\t', index / u)
                }
                Indentation::Spaces(u) => {
                    (' ', index * u)
                }
            };
            new.push_str(format!("{:width$}", char, width = width).as_str());
            new.push_str(&buf[index..]);
            new
        } else {
            buf
        }
    }
}
