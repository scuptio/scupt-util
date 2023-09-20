use std::sync::Once;

use console_subscriber;
use env_logger;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

static INIT: Once = Once::new();

static INIT_ENV: Once = Once::new();

pub fn logger_setup_with_console() {
    INIT.call_once(
        || { _setup_with_console(); }
    );
}

pub fn _setup_with_console() {
    let console_layer = console_subscriber::spawn();
    tracing_subscriber::registry()
        .with(console_layer)
        .with(
            tracing_subscriber::fmt::layer()
                .with_level(true)
                .with_ansi(false)
                .with_file(true)
                // display source code line numbers
                .with_line_number(true)
                .without_time()
                .with_filter(
                    tracing_subscriber::filter::LevelFilter::DEBUG
                ),
        )
        .init();
}

/// Setup function that is only run once, even if called multiple times.
pub fn logger_setup() {
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            // enable info
            .with_max_level(tracing::Level::INFO)
            // display source code file paths
            .with_file(true)
            // display source code line numbers
            .with_line_number(true)
            // disable targets
            .with_target(false)
            // sets this to be the default, global collector for this application.
            .init();
    });
}

pub fn log_env_setup() {
    INIT_ENV.call_once(|| {
        env_logger::init();
    });
}