use crate::init_logger::{_setup_with_console, INIT};

pub fn logger_setup(level:&str) {
    INIT.call_once(
        || { _setup_with_console(level, false); }
    );
}

pub fn logger_setup_with_console(level:&str) {
    INIT.call_once(
        || { _setup_with_console(level, true); }
    );
}