use clap::{Arg, Command, arg};

pub fn cli() -> Command {
    Command::new("star")
        .about("tar based command line utility in rust")
        .subcommand_required(true)
        .subcommand(
            Command::new("-xzf")
                .about("Extracts  files")
                .arg(arg!(<FILE> "File to extract").required(false))
                .arg_required_else_help(true)
                
        )
        .subcommand(
            Command::new("search")
        )
}