mod types;

use std::{fmt, result::Result};
use structopt::StructOpt;
use types::Opt;


/// sanity check input options to ensure LaTeX specific options are used in conjunction with the
/// LaTeX option.
fn input_sanitize(opt: &Opt) -> Result<(), &'static str> {
    if opt.colorize || !opt.latex {
        Err("The `--colorize | -c` option is only available with the `--latex | -l` option.")
    } else {
        Ok(())
    }
}

fn main() {
    let opt = Opt::from_args();

    // sanity check input options.
    let maybe_sanitized = 
}
