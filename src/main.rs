use std::{fs::File, io::Read};

use anyhow::Context;

use dialoguer::{BasicHistory, Input};
use tracing::{debug, error, info};

use crate::{
    core::{
        cli::{Args, parse_args},
        log::init_logging,
    },
    dbms::DBMS,
    ql::{
        parser::{Parser, statement::QLStatement},
        tokenizer::{errors::SQLTokenizeError, token::Token, tokenizer::Tokenizer},
    },
};

mod core;
mod dbms;
mod ql;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging()?;
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
    dbms.load_schemas()
        .map_err(|e| {
            error!("Error loading DBMS: {}", e);
            e
        })
        .context("Error loading DBMS")?;

    if let Some(sql_file_path) = &args.sql_file {
        let mut file = File::options()
            .read(true)
            .open(sql_file_path)
            .map_err(|e| {
                error!("Error opening SQL file for reading: {}", e);
                e
            })?;
        let mut sql = String::new();
        file.read_to_string(&mut sql).map_err(|e| {
            error!("Error reading SQL file: {}", e);
            e
        })?;

        let tokens: Vec<Token> = tokenize_str(&sql, sql_file_path)?;
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
        let mut history: BasicHistory = BasicHistory::new().max_entries(10).no_duplicates(true);
        loop {
            let input: String = Input::new()
                .history_with(&mut history)
                .with_prompt("#")
                .interact_text()?;
            if let Some(input) = input.strip_prefix("\\") {
                match input {
                    "q" => break,
                    "ddm" => println!("{:#?}", &dbms),
                    "ss" => dbms.save_schemas().map_or_else(
                        |e| {
                            error!("Error saving schemas: {}", e);
                            ()
                        },
                        |_| println!("Database schemas saved"),
                    ),
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
