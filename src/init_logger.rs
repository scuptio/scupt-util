use std::sync::Once;

use console_subscriber;
use env_logger;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub static INIT: Once = Once::new();

static INIT_ENV: Once = Once::new();

pub fn logger_setup_with_console() {
    INIT.call_once(
        || { _setup_with_console("info", true); }
    );
}


pub fn _setup_with_console(level:&str, enable_console_layer:bool) {
    let filter = match level {
        "info" => { tracing_subscriber::filter::LevelFilter::INFO }
        "debug" => { tracing_subscriber::filter::LevelFilter::DEBUG }
        "trace" => { tracing_subscriber::filter::LevelFilter::TRACE }
        "warn" => { tracing_subscriber::filter::LevelFilter::WARN }
        "error" => { tracing_subscriber::filter::LevelFilter::ERROR }
        _ => { panic!("unknown level {}", level)}
    };
    let register = tracing_subscriber::registry();
    if enable_console_layer {
        let console_layer = console_subscriber::spawn();
        register.with(console_layer).with(
            tracing_subscriber::fmt::layer()
                .with_level(true)
                .with_ansi(false)
                .with_file(true)
                // display source code line numbers
                .with_line_number(true)
                .without_time()
                .with_filter(filter),
        )
            .init();
    } else {
        register.with(
            tracing_subscriber::fmt::layer()
                .with_level(true)
                .with_ansi(false)
                .with_file(true)
                // display source code line numbers
                .with_line_number(true)
                .without_time()
                .with_filter(filter),
        )
            .init();
    };


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