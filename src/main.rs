use std::{fs::File, io::Read};

use anyhow::Context;
use clap::Parser;
use dialoguer::Input;
use env_logger::{Builder, Env, TimestampPrecision};
use log::{debug, error, info};

use crate::{
    dbms::DBMS,
    ql::tokenizer::{errors::SQLTokenizeError, token::Token, tokenizer::Tokenizer},
};

mod db;
mod dbms;
mod ql;
mod row;
mod table;
mod util;

#[derive(Debug, Parser)]
struct Args {
    #[arg(
        long = "dbg_print_tokens_file",
        long_help = "Debug prints the tokenized results of a sql file"
    )]
    dbg_print_file_tokens: bool,

    #[arg(
        long = "dbg_print_tokens",
        long_help = "Debug prints the tokenized results from stdin"
    )]
    dbg_print_tokens: bool,

    #[arg(
        long = "create_root",
        long_help = "Creates the dbms root if it does not exist"
    )]
    create_dbms_root: bool,

    #[arg(long= "db_root", long_help = "The root path for the DBMS", default_value_t = String::from("dbms_root"))]
    db_root: String,

    #[arg(long = "file", long_help = "The sql file to run")]
    sql_file: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_file(true)
        .format_line_number(true)
        .format_timestamp(Some(TimestampPrecision::Millis))
        .format_target(false)
        .init();
    let args = Args::parse();
    info!("Starting dbms v{}", env!("CARGO_PKG_VERSION"));

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
        if args.dbg_print_file_tokens {
            debug!("{:#?}", tokens);
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
        }
    }

    Ok(())
}

fn tokenize_str(str: &str, source_file: &str) -> Result<Vec<Token>, SQLTokenizeError> {
    let mut tokenizer = Tokenizer::new(str);
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
