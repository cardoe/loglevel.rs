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
        let mut other_modules = Vec::new();
        for i in 0..10_000 {
            other_modules.push(format!("other::mod{}", i));
        }
        other_modules.push(format!("{}::common", module_path!()));
        stderrlog::StdErrLog::new()
            .timestamp(stderrlog::Timestamp::Second)
            .verbosity(10)
            .modules(other_modules)
            .init()
            .unwrap();
    });
}
