use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use anyhow::Result;
use paw;
use tempfile::NamedTempFile;

mod args;
mod indentation;
mod new_line;

use args::Args;

#[paw::main]
fn main(args: Args) -> Result<()> {
    let input = File::open(&args.input)?;
    let newline = args.target_newline;
    let indentation = args.indentation_style;
    let temp = NamedTempFile::new()?;
    let mut buffer = String::new();

    let mut reader = BufReader::new(input);
    let mut writer = BufWriter::new(&temp);

    while let Ok(line) = reader.read_line(&mut buffer) {
        if line == 0 {
            // We have reached EOF
            break;
        }

        if let Some(newline) = &newline {
            buffer = newline.make_transformation(&buffer);
        }

        if let Some(indentation) = &indentation {
            buffer = indentation.make_transformation(buffer);
        }

        std::io::stdout().write_all(buffer.as_bytes())?;
        buffer.clear();
    }
    std::fs::copy(temp.path(), "output.txt");
    Ok(())
}
