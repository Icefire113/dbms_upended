use std::{fs::File, io::Read};

use clap::Parser;
use env_logger::{Builder, Env, TimestampPrecision};
use log::{debug, error, info};

use crate::ql::tokenizer::{errors::SQLTokenizeError, token::Token, tokenizer::Tokenizer};

mod db;
mod dbms;
mod ql;
mod row;
mod table;
mod util;

#[derive(Debug, Parser)]
struct Args {
    #[arg(
        long = "dbg_print_tokens",
        long_help = "Debug prints the tokenized results of a sql file"
    )]
    debug_print_tokens: bool,

    #[arg(long= "db_root", long_help = "The root path for the DBMS", default_value_t = String::from("dbms_root"))]
    db_root: String,

    #[arg(long_help = "The sql file to run")]
    sql_file: String,
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

    let mut sql_file = File::options()
        .read(true)
        .open(&args.sql_file)
        .map_err(|e| {
            error!("Error opening SQL file for reading: {:#?}", e);
            e
        })?;
    let mut sql = String::new();
    sql_file.read_to_string(&mut sql).map_err(|e| {
        error!("Error reading SQL file: {:#?}", e);
        e
    })?;

    let mut tokenizer = Tokenizer::new(&sql);
    let tokens: Vec<Token> = tokenizer.tokenize().map_err(|e| {
        match &e {
            SQLTokenizeError::IllegalToken(tok, line, col) => {
                error!("Illegal token `{tok}` at: {}:{line}:{col}", args.sql_file)
            }
            SQLTokenizeError::UnknownToken(tok, line, col) => {
                error!("Unknown token `{tok}` at: {}:{line}:{col}", args.sql_file)
            }
        };
        e
    })?;
    if args.debug_print_tokens {
        debug!("{:#?}", tokens);
    }

    Ok(())
}
