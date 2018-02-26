#![feature(test)]
extern crate libc;
extern crate stderrlog;
extern crate test;

#[macro_use]
extern crate log;

mod util;
mod common;

use std::sync;

static INIT_LOGGER: sync::Once = sync::ONCE_INIT;

fn init_logger() {
    INIT_LOGGER.call_once(|| {
        stderrlog::StdErrLog::new()
            .timestamp(stderrlog::Timestamp::Second)
            .verbosity(10)
            .module(module_path!())
            .init()
            .unwrap();
    });
}
