use std::{fs::File, sync::Mutex};

use anyhow::Context;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logging() -> anyhow::Result<()> {
    let log_file = File::create("dbms.log").context("Creating log file")?;
    let file_layer = fmt::layer()
        .with_file(true)
        .with_target(false)
        .with_line_number(true)
        .with_ansi(false)
        .with_writer(Mutex::new(log_file));

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let stderr_layer = fmt::layer()
        .with_file(true)
        .with_ansi(true)
        .with_line_number(true)
        .with_target(false)
        .with_writer(std::io::stderr)
        .with_filter(env_filter);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(stderr_layer)
        .init();
    Ok(())
}
