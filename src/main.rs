mod types;
mod color;

use std::{fmt, result::Result};
use structopt::StructOpt;
use types::Opt;
use color::get_hex;


/// sanity check input options to ensure LaTeX specific options are used in conjunction with the
/// LaTeX option.
fn input_sanitize(opt: &Opt) -> Result<(), &'static str> {
    if !opt.colorize || !opt.latex || !opt.newlines {
        Ok(())
    }
    else if opt.colorize || !opt.latex {
        Err("The `--colorize | -c` option is only available with the `--latex | -l` option.")
    } else {
        Ok(())
    }
}

fn main() {
    let opt = Opt::from_args();

    let mut color_state = color::ColorFSM { state: 0 };

    // sanity check input options.
    let maybe_sanitized = input_sanitize(&opt);
    if maybe_sanitized.is_err() {
        eprintln!("ERROR: {}", maybe_sanitized.unwrap_err());
    }

    get_hex(&mut color_state);

}
