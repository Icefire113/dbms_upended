use std::io;

use clap::{Arg, ArgAction, Command, ValueHint, command, value_parser};
use clap_complete::{Generator, Shell, generate};

#[derive(Debug)]
pub struct Args {
    pub dbg_print_tokens: bool,
    pub dbg_print_queries: bool,
    pub create_dbms_root: bool,
    pub db_root: String,
    pub sql_file: Option<String>,
}

fn build_cli() -> clap::Command {
    command!()
        .arg(
            clap::Arg::new("dbg_print_tokens")
                .long("dbg_print_tokens")
                .long_help("Debug prints the tokenized results")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("dbg_print_queries")
                .long("dbg_print_queries")
                .long_help("Debug prints the parsed queries")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("create_dbms_root")
                .long("create_root")
                .long_help("Creates the dbms root if it does not exist")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("db_root")
                .long("db_root")
                .value_hint(ValueHint::DirPath)
                .long_help("The root path for the DBMS")
                .default_value("dbms_root"),
        )
        .arg(
            clap::Arg::new("sql_file")
                .value_hint(ValueHint::FilePath)
                .long("file")
                .long_help("The sql file to run"),
        )
        .arg(
            Arg::new("generator")
                .long("generate")
                .long_help("Used to generate shell completions, can be used like so: `dbms_upended --generate bash > /usr/share/bash-completion/completions/dbms_upended.bash`")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        )
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

pub fn parse_args() -> Option<Args> {
    let matches = build_cli().get_matches();

    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = build_cli();
        print_completions(generator, &mut cmd);
        return None;
    }

    Some(Args {
        dbg_print_tokens: matches.get_flag("dbg_print_tokens"),
        dbg_print_queries: matches.get_flag("dbg_print_queries"),
        create_dbms_root: matches.get_flag("create_dbms_root"),
        db_root: matches.get_one::<String>("db_root").unwrap().clone(),
        sql_file: matches.get_one("sql_file").cloned(),
    })
}
