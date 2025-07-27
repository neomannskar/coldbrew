use clap::{Arg, ArgMatches, Command};

pub fn parse() -> ArgMatches {
    Command::new("coldbrew")
        .version("0.1.0")
        .author("Neo Mannskär <neo@mannskar.com>")
        .about("Package manager and build tool for the Lotus programming language")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
                .about("Create a new Lotus project")
                .arg(
                    Arg::new("name")
                        .help("Name of the project")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("build")
                .about("Build the current Lotus project"),
        )
        .subcommand(
            Command::new("run")
                .about("Build and run the current Lotus project"),
        )
        .subcommand(
            Command::new("add")
                .about("Add a dependency")
                .arg(
                    Arg::new("package") // fixed: was "order"
                        .help("Package name to add")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("install")
                .about("Install packages from https://lotuslang.com/orders"),
        )
        .subcommand(
            Command::new("publish")
                .about("Publish this package to the Lotus order registry"),
        )
        .get_matches()
}
