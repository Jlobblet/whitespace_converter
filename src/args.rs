use std::path::PathBuf;

use structopt::StructOpt;

use crate::indentation::Indentation;
use crate::new_line::NewLine;

#[derive(Debug, StructOpt)]
#[structopt(name = "wsc", about = "whitespace converter")]
pub(crate) struct Args {
    #[structopt(parse(from_os_str))]
    pub(crate) input: PathBuf,

    #[structopt(short="n", long="newline")]
    pub(crate) target_newline: Option<NewLine>,

    #[structopt(short="i", long="indentation")]
    pub(crate) indentation_style: Option<Indentation>,
}
