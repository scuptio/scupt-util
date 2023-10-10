use crate::init_logger::{_setup_with_console, INIT};

pub fn logger_setup(level:&str) {
    INIT.call_once(
        || { _setup_with_console(level); }
    );
}