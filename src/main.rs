use anyhow::Ok;
use clap::Parser;
use tracing::span;
use tracing_subscriber::{
    Layer, Registry, fmt::writer::MakeWriterExt, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::config::{AppConfig, LogConfig};

mod config;
mod models;
mod routes;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(short, long, help = "path to config file")]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut config_builder = ::config::Config::builder();
    if let Some(config_path) = args.config {
        config_builder = config_builder.add_source(::config::File::with_name(&config_path));
    }

    config_builder =
        config_builder.add_source(::config::Environment::with_prefix("LIVESERVER").separator("_"));

    let app_conf: AppConfig = config_builder.build()?.try_deserialize()?;

    init_logger(&app_conf.log)?;

    let _span = span!(tracing::Level::TRACE, "app");
    let _ = _span.enter();

    // TODO

    Ok(())
}

fn init_logger(log_config: &LogConfig) -> anyhow::Result<()> {
    let log_level = LogConfig::to_level(log_config.level.as_ref());
    let log_file = &log_config.path;

    let mut layers = Vec::new();
    // init console logger
    if log_config.console.unwrap_or(true) {
        let layer = tracing_subscriber::fmt::layer()
            .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
            .with_writer(
                std::io::stdout.with_max_level(
                    log_config
                        .console_level
                        .as_ref()
                        .map_or(log_level, LogConfig::str_to_level),
                ),
            );
        let layer = match log_config
            .format
            .as_deref()
            .unwrap_or("pretty")
            .to_lowercase()
            .as_str()
        {
            "compact" => layer.compact().boxed(),
            "full" => layer.boxed(),
            "json" => layer.json().boxed(),
            _ => layer.pretty().boxed(),
        };
        layers.push(layer);
    }

    // init file json logger
    if let Some(f) = log_file {
        let path = std::path::Path::new(&f);
        let directory = path.parent().expect("failed to parse log file");

        if !std::path::Path::exists(directory) {
            std::fs::create_dir_all(directory).expect("failed to create log directory");
        }

        let wrt = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(path)
            .expect("failed to open log file");

        let layer = tracing_subscriber::fmt::layer()
            .json()
            .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
            .with_file(true)
            .with_line_number(true)
            .with_writer(wrt.with_max_level(log_level))
            .boxed();

        layers.push(layer);
    }

    let layers = layers
        .into_iter()
        .reduce(|acc, layer| acc.and_then(layer).boxed());

    if let Some(layers) = layers {
        Registry::default().with(layers).init();
    }

    tracing::debug!(target: "init_logger","logger initialized");

    Ok(())
}
