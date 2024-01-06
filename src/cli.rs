use clap::{Command, Arg, arg};

pub fn cli() -> Command {
    Command::new("star")
        .about("Tar based command line utility written in rust")
        .subcommand_required(true)
        .subcommand(
            Command::new("-gz")
                .about("Extracts .gz files")
                .arg(arg!(<FILE> "File to extract").required(false))
                .arg(arg!(-v --verbose "Print verbose output"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("-xz")
                .about("Extracts .xz files")
                .arg(arg!(<FILE> "File to extract").required(false))
                .arg(arg!(-v --verbose "Print verbose output"))
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("-search")
                .about("Searches for files containing a keyword")
                .arg(arg!(<KEYWORD> "Keyword").required(true))
                .arg(arg!(-v --verbose "Print verbose output"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("-bz2")
                .about("Extracts .bz2 files")
                .arg(arg!(<FILE> "File to extract").required(true))
                .arg(arg!(-v --verbose "Print verbose output"))
                .arg_required_else_help(true),
        )
}