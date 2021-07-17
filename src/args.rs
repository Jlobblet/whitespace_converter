use std::path::PathBuf;

use structopt::StructOpt;

use crate::indentation::Indentation;
use crate::new_line::NewLine;

#[derive(Debug, StructOpt)]
#[structopt(name = "wsc")]
/// \[w\]hite\[s\]pace \[c\]onverter
///
/// Convert whitespace in a given file and output to stdout.
pub(crate) struct Args {
    /// Path to input file to convert.
    ///
    /// Only a single file is supported.
    #[structopt(parse(from_os_str))]
    pub(crate) input: PathBuf,

    /// Whether to convert to LF or CRLF line endings.
    ///
    /// If nothing is passed, line endings are not converted.
    /// Pass LF for \n and CRLF for \r\n.
    ///
    /// e.g wsc README.md -n LF
    ///
    /// wsc README.md -n CRLF
    #[structopt(short="n", long="newline")]
    pub(crate) target_newline: Option<NewLine>,

    /// Whether to convert indentation to tabs or spaces.
    ///
    /// Pass \[tabs|spaces\]=number to specify how many spaces there should be per tab during conversion.
    ///
    /// e.g. wsc README.md -i tabs=4
    ///
    /// wsc README.md -i spaces=4
    #[structopt(short="i", long="indentation")]
    pub(crate) indentation_style: Option<Indentation>,
}
