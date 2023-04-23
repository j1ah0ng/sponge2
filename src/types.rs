use structopt::StructOpt;

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
}
