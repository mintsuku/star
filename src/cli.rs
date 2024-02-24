use clap::{Command, arg};

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

        .subcommand(
            Command::new("-c")
                .about("Creates a .tar file")
                .arg(arg!(<TYPE> "File type to create").required(true))
                .arg(arg!(<FILE> "File to create").required(true))
                .arg(arg!(-v --verbose "Print verbose output"))
                .arg_required_else_help(true),
               
        )

        .subcommand(
            Command::new("-l")
                .about("Lists the contents of a .tar file")
                .arg(arg!(<FILE> "File to list").required(true))
                .arg_required_else_help(true),
        )
}