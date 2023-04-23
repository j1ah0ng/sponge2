use structopt::StructOpt;
use std::path::PathBuf;

/// Command line options.
#[derive(StructOpt, Debug)]
#[structopt(name = "cmdline")]
pub struct Opt {
    /// Emit LaTeX.
    #[structopt(short, long)]
    pub latex: bool,

    /// Emit LaTeX in color.
    #[structopt(short, long)]
    pub colorize: bool,

    /// Newline after each word.
    #[structopt(short, long)]
    pub newlines: bool,

    /// Input file.
    #[structopt(short, long, parse(from_os_str))]
    pub file: Option<PathBuf>,

    /// Words to sponge.
    #[structopt(name = "CONTENT")]
    pub content: Vec<String>,

}

/// Modes of operation.
#[derive(Eq, PartialEq, Debug)]
pub enum Mode {
    Stdin, FromFile, FromArgv
}

/// State machine. 
#[derive(Debug)]
pub struct CaseStateMachine {
    /// Consecutive uppercase
    up: u32,

    /// Consecutive lowercase
    down: u32,
}

impl CaseStateMachine {
    pub fn new() -> Self {
        CaseStateMachine {
            up: 0,
            down: 0,
        }
    }

    pub fn up(&self) -> u32 {
        self.up
    }

    pub fn down(&self) -> u32 {
        self.down
    }

    pub fn add_up(&mut self) {
        self.up += 1;
        self.zero();
    }
    pub fn add_down(&mut self) {
        self.down += 1;
        self.zero();
    }

    fn zero(&mut self) {
        if self.up == self.down {
            self.up = 0;
            self.down = 0;
        }
    }

    pub fn next_is_uppercase(&mut self) -> bool {
        match self.up < self.down {
            true => {
                self.add_up();
                true
            },
            false => {
                self.add_down();
                false
            },
        }
    }

}
