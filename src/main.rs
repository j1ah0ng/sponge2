mod types;
mod clip;
mod color;
mod default;

use std::{fmt, io::stdin, io::Read, result::Result};
use structopt::StructOpt;
use types::{Opt, CaseStateMachine};
use color::get_hex;
use clip::clip;

/// determine what mode to use
fn get_mode(opt: &Opt) -> types::Mode {
    if opt.file.is_some() {
        types::Mode::FromFile
    } else if opt.content.len() > 0 {
        types::Mode::FromArgv
    } else {
        types::Mode::Stdin
    }
}

/// sanity check input options to ensure LaTeX specific options are used in conjunction with the
/// LaTeX option.
fn input_sanitize(opt: &Opt) -> Result<(), &'static str> {
    if opt.colorize && !opt.latex {
        Err("The `--colorize | -c` option is only available with the `--latex | -l` option.")
    } else {
        Ok(())
    }
}

/// sponge a string
fn do_sponge(instring: &String, state: &mut CaseStateMachine) -> String {
    instring.chars().map(|c| {
        match c {
            'C' => {
                state.add_down();
                String::from("c")
            },
            'I' => {
                state.add_down();
                String::from("i")
            },
            'l' => {
                state.add_up();
                String::from("L")
            },
            default => {
                match state.next_is_uppercase() {
                    true => c.to_uppercase().collect::<String>(),
                    false => c.to_lowercase().collect::<String>(),
                }
            },
        }
    }).collect()
}

fn main() {
    let opt = Opt::from_args();

    let mut color_state = color::ColorFSM { state: 0 };

    // sanity check input options.
    let maybe_sanitized = input_sanitize(&opt);
    if maybe_sanitized.is_err() {
        eprintln!("ERROR: {}", maybe_sanitized.unwrap_err());
    }

    println!("{}", get_hex(&mut color_state));
    println!("{}", get_hex(&mut color_state));

    // proceed
    let mode = get_mode(&opt);
    if mode == types::Mode::Stdin {

        // Read from stdin until EOF
        let mut state = types::CaseStateMachine::new();
        let mut stdin = stdin();
        let mut instring = String::new();
        while let Ok(n) = stdin.read_to_string(&mut instring) {
            if n == 0 {
                return
            } else {
                let outstring: String = do_sponge(&instring, &mut state);
                print!("{}", outstring);
            }

        }

    } else {

        // Get the input as a string, all at once
        let instring = match mode {
            types::Mode::FromFile => std::fs::read_to_string(opt.file.unwrap()),
            types::Mode::FromArgv => Ok(opt.content.join(" ")),
            default => panic!("not reachable")
        }.unwrap();

        // Sponge it
        let mut state = types::CaseStateMachine::new();
        let outstring: String = do_sponge(&instring, &mut state);

        println!("{}", outstring);

        clip(outstring);
    }
}
