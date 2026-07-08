use std::{
    fs::File,
    io::{self, Read},
    process::exit,
};

use anyhow::Context;
use clap::{Arg, ArgAction, Command, ValueHint, command, value_parser};
use clap_complete::{Generator, Shell, generate};
use dialoguer::Input;
use env_logger::{Builder, Env, TimestampPrecision};
use log::{debug, error, info};

use crate::{
    dbms::DBMS,
    ql::{
        parser::{Parser, statement::QLStatement},
        tokenizer::{errors::SQLTokenizeError, token::Token, tokenizer::Tokenizer},
    },
};

mod db;
mod dbms;
mod ql;
mod row;
mod table;
mod util;

#[derive(Debug)]
struct Args {
    dbg_print_tokens: bool,
    dbg_print_queries: bool,
    create_dbms_root: bool,
    db_root: String,
    sql_file: Option<String>,
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

fn parse_args() -> Option<Args> {
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_file(true)
        .format_line_number(true)
        .format_timestamp(Some(TimestampPrecision::Millis))
        .format_target(false)
        .init();
    let args: Args = match parse_args() {
        Some(args) => args,
        // If we return none, then the arg parser has run a once out command, and we should exit
        None => return Ok(()),
    };
    info!(
        "Starting dbms v{}+{}",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_HASH")
    );

    let mut dbms = DBMS::new(args.db_root, args.create_dbms_root)?;
    dbms.load()
        .map_err(|e| {
            error!("Error loading DBMS: {}", e);
            e
        })
        .context("Error loading DBMS")?;

    if let Some(sql_file) = &args.sql_file {
        let mut file = File::options().read(true).open(sql_file).map_err(|e| {
            error!("Error opening SQL file for reading: {}", e);
            e
        })?;
        let mut sql = String::new();
        file.read_to_string(&mut sql).map_err(|e| {
            error!("Error reading SQL file: {}", e);
            e
        })?;

        let tokens = tokenize_str(&sql, sql_file)?;
        if args.dbg_print_tokens {
            debug!("{:#?}", tokens);
        }
        let parser: Parser<'_> = Parser::new(&tokens);
        let queries: Vec<QLStatement> = parser.parse().map_err(|e| {
            error!("Error parsing SQL: {}", e);
            e
        })?;

        if args.dbg_print_queries {
            debug!("{:#?}", queries);
        }
    } else {
        // read sql from stdin in a loop
        println!("Enter SQL, use \\q to quit");
        loop {
            let input: String = Input::new().with_prompt("#").interact_text()?;
            if let Some(input) = input.strip_prefix("\\") {
                match input {
                    "q" => break,
                    "ddm" => println!("{:#?}", &dbms),
                    other => println!("Unknown command '{other}'"),
                }
                continue;
            }

            let tokens = match tokenize_str(&input, "<stdin>") {
                Ok(tokens) => tokens,
                Err(_) => {
                    continue;
                }
            };

            if args.dbg_print_tokens {
                debug!("{:#?}", tokens);
            }

            let parser: Parser<'_> = Parser::new(&tokens);
            let queries: Vec<QLStatement> = match parser.parse() {
                Ok(queries) => queries,
                Err(e) => {
                    error!("Error parsing SQL: {}", e);
                    continue;
                }
            };

            if args.dbg_print_queries {
                debug!("{:#?}", queries);
            }
        }
    }

    Ok(())
}

fn tokenize_str(str: &str, source_file: &str) -> Result<Vec<Token>, SQLTokenizeError> {
    let mut tokenizer: Tokenizer<'_> = Tokenizer::new(str);
    let tokens: Vec<Token> = tokenizer.tokenize().map_err(|e| {
        match &e {
            SQLTokenizeError::IllegalToken(tok, line, col) => {
                error!("Illegal token `{tok}` at: {source_file}:{line}:{col}")
            }
            SQLTokenizeError::UnknownToken(tok, line, col) => {
                error!("Unknown token `{tok}` at: {source_file}:{line}:{col}")
            }
        };
        e
    })?;
    Ok(tokens)
}
