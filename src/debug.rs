// src/debug.rs
use std::fs::OpenOptions;
use std::io::Write;

const DEBUG_LOG: &str = "/tmp/tmuxgotchi-debug.log";

pub fn log(msg: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(DEBUG_LOG)
    {
        let _ = writeln!(file, "[{}] {}", chrono::Local::now().format("%H:%M:%S%.3f"), msg);
    }
}

#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        $crate::debug::log(&format!($($arg)*))
    };
}
